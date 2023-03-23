use database::DbConnection;
use domain::model::{
    FaqCategory, FaqCategoryContent, FaqCategoryId, FaqCategoryItem, FaqCategoryItemWithCategory,
    FaqCategoryWithContents, FaqItem, FaqItemContent, FaqItemId, FaqItemWithContentsAndCategories,
    FaqSettings, FaqSettingsData, PagingResult, Slug,
};
use domain::repository::FaqRepository;

pub struct FaqRepositoryImpl;

impl FaqRepository for FaqRepositoryImpl {
    type Err = domain::Error;
    type Transaction = DbConnection;

    fn get_or_create_default_settings(
        &self,
        tx: &mut Self::Transaction,
    ) -> Result<FaqSettings, Self::Err> {
        if let Some(entity) = database::adapters::faq_settings::get(tx)? {
            Ok(entity.into())
        } else {
            let new_model = FaqSettings::default();
            database::adapters::faq_settings::create(tx, (&new_model).into())?;
            Ok(new_model)
        }
    }

    fn update_settings(
        &self,
        tx: &mut Self::Transaction,
        settings: &mut FaqSettings,
        data: FaqSettingsData,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_settings::update(tx, &(&data).into(), &settings.id)?;
        settings.data = data;
        Ok(())
    }

    fn create_category(
        &self,
        tx: &mut Self::Transaction,
        category: &FaqCategory,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_category::create(tx, category.into())?;
        Ok(())
    }

    fn next_category_display_order(&self, tx: &mut Self::Transaction) -> Result<u32, Self::Err> {
        let current_max = database::adapters::faq_category::get_max_display_order(tx)?.unwrap_or(0);
        Ok(current_max + 1)
    }

    fn create_category_content(
        &self,
        tx: &mut Self::Transaction,
        category_content: &FaqCategoryContent,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_category_content::create(tx, category_content.into())?;
        Ok(())
    }

    fn get_category_by_slug(
        &self,
        tx: &mut Self::Transaction,
        slug: &Slug,
    ) -> Result<Option<FaqCategory>, Self::Err> {
        let entity = database::adapters::faq_category::get_by_slug(tx, &slug)?;
        Ok(entity.map(Into::into))
    }

    fn search_categories_by_text(
        &self,
        tx: &mut Self::Transaction,
        text: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<PagingResult<FaqCategoryWithContents>, Self::Err> {
        let (total, category_entities) = database::adapters::faq_category::search_by_text(
            tx,
            text,
            limit as i64,
            offset as i64,
        )?;
        let category_ids: Vec<&str> = category_entities
            .iter()
            .map(|category| category.id.as_str())
            .collect();
        let content_entities =
            database::adapters::faq_category_content::get_list_by_faq_category_ids(
                tx,
                &category_ids,
            )?;
        let list = category_entities
            .into_iter()
            .map(|category_entity| {
                let contents = content_entities
                    .iter()
                    .filter(|c| c.faq_category_id.as_str() == category_entity.id.as_str())
                    .map(FaqCategoryContent::from)
                    .collect();
                let category = FaqCategory::from(category_entity);
                FaqCategoryWithContents::from((category, contents))
            })
            .collect();
        Ok(PagingResult {
            total: total as u64,
            list,
        })
    }

    fn get_category_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &FaqCategoryId,
    ) -> Result<Option<FaqCategory>, Self::Err> {
        let entity = database::adapters::faq_category::get_by_id(tx, &id)?;
        Ok(entity.map(Into::into))
    }

    fn get_category_with_contents_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &FaqCategoryId,
    ) -> Result<Option<FaqCategoryWithContents>, Self::Err> {
        let entity = database::adapters::faq_category::get_by_id(tx, &id)?;
        if entity.is_none() {
            return Ok(None);
        }
        let category = FaqCategory::from(entity.unwrap());
        let content_entities =
            database::adapters::faq_category_content::get_list_by_faq_category_id(tx, &id)?;
        let contents: Vec<FaqCategoryContent> =
            content_entities.into_iter().map(Into::into).collect();
        Ok(Some((category, contents).into()))
    }

    fn update_category_with_contents(
        &self,
        tx: &mut Self::Transaction,
        category_with_contents: &mut FaqCategoryWithContents,
        slug: Slug,
        contents: Vec<FaqCategoryContent>,
    ) -> Result<(), Self::Err> {
        // delete and creates contents
        database::adapters::faq_category_content::delete_by_faq_category_id(
            tx,
            &category_with_contents.id,
        )?;
        for content in &contents {
            database::adapters::faq_category_content::create(tx, content.into())?;
        }
        // update slug
        database::adapters::faq_category::update_slug_by_id(tx, &slug, &category_with_contents.id)?;
        category_with_contents.slug = slug;
        Ok(())
    }

    fn delete_category_with_contents(
        &self,
        tx: &mut Self::Transaction,
        category_with_contents: FaqCategoryWithContents,
    ) -> Result<(), Self::Err> {
        let deleted_display_order = category_with_contents.display_order;
        database::adapters::faq_category::delete_by_id(tx, &category_with_contents.id)?;
        database::adapters::faq_category::decrement_display_order_by_from_display_order(
            tx,
            deleted_display_order,
        )?;
        Ok(())
    }

    fn reorder_faq_category(
        &self,
        tx: &mut Self::Transaction,
        objective: FaqCategory,
        target: FaqCategory,
        append: bool,
    ) -> Result<(), Self::Err> {
        if target.id == objective.id || target.display_order == objective.display_order {
            return Ok(());
        }
        let pre_display_order = objective.display_order;
        let next_display_order = if objective.display_order < target.display_order {
            let next_display_order = if append {
                target.display_order
            } else {
                target.display_order - 1
            };
            database::adapters::faq_category::decrement_display_order_by_range(
                tx,
                pre_display_order + 1,
                next_display_order,
            )?;
            next_display_order
        } else {
            let next_display_order = if append {
                target.display_order + 1
            } else {
                target.display_order
            };
            database::adapters::faq_category::increment_display_order_by_range(
                tx,
                next_display_order,
                pre_display_order - 1,
            )?;
            next_display_order
        };
        database::adapters::faq_category::update_display_order_by_id(
            tx,
            next_display_order,
            &objective.id,
        )?;
        Ok(())
    }

    fn create_item(&self, tx: &mut Self::Transaction, item: &FaqItem) -> Result<(), Self::Err> {
        database::adapters::faq_item::create(tx, item.into())?;
        Ok(())
    }

    fn get_item_by_slug(
        &self,
        tx: &mut Self::Transaction,
        slug: &Slug,
    ) -> Result<Option<FaqItem>, Self::Err> {
        let entity = database::adapters::faq_item::get_by_slug(tx, &slug)?;
        Ok(entity.map(Into::into))
    }

    fn get_item_with_contents_and_categories_by_id(
        &self,
        tx: &mut Self::Transaction,
        id: &FaqItemId,
    ) -> Result<Option<FaqItemWithContentsAndCategories>, Self::Err> {
        let faq_item_entity = database::adapters::faq_item::get_by_id(tx, &id)?;
        if faq_item_entity.is_none() {
            return Ok(None);
        }
        let faq_item = FaqItem::from(faq_item_entity.unwrap());
        let content_entities =
            database::adapters::faq_item_content::get_list_by_faq_item_id(tx, &id)?;
        let contents: Vec<FaqItemContent> = content_entities.into_iter().map(Into::into).collect();
        let faq_category_item_entities =
            database::adapters::faq_category_item::get_list_by_faq_item_id(tx, &id)?;
        let faq_category_ids: Vec<&str> = faq_category_item_entities
            .iter()
            .map(|e| e.faq_category_id.as_str())
            .collect();
        let faq_category_entities =
            database::adapters::faq_category::get_list_by_ids(tx, &faq_category_ids)?;
        let faq_category_content_entities =
            database::adapters::faq_category_content::get_list_by_faq_category_ids(
                tx,
                &faq_category_ids,
            )?;
        let categories: Vec<FaqCategoryItemWithCategory> = faq_category_item_entities
            .into_iter()
            .map(|category_item_entity| {
                let category_item = FaqCategoryItem::from(category_item_entity);
                let category: FaqCategory = faq_category_entities
                    .iter()
                    .find(|e| e.id.as_str() == category_item.faq_category_id.as_str())
                    .unwrap()
                    .into();
                let category_contents: Vec<FaqCategoryContent> = faq_category_content_entities
                    .iter()
                    .filter(|e| {
                        e.faq_category_id.as_str() == category_item.faq_category_id.as_str()
                    })
                    .map(Into::into)
                    .collect();
                let category_with_contents =
                    FaqCategoryWithContents::from((category, category_contents));
                FaqCategoryItemWithCategory::from((category_item, category_with_contents))
            })
            .collect();
        Ok(Some((faq_item, contents, categories).into()))
    }

    fn delete_item_with_contents_and_categories(
        &self,
        tx: &mut Self::Transaction,
        item_with_contents_and_categories: FaqItemWithContentsAndCategories,
    ) -> Result<(), Self::Err> {
        // delete
        database::adapters::faq_item::delete_by_id(tx, &item_with_contents_and_categories.id)?;
        // update faq_category_items.display_order
        for category_item in item_with_contents_and_categories.categories {
            database::adapters::faq_category_item::decrement_display_order_by_faq_category_id_and_from_display_order(
                tx,
                category_item.display_order,
                &category_item.faq_category_id,
            )?;
        }
        Ok(())
    }

    fn create_item_content(
        &self,
        tx: &mut Self::Transaction,
        item_content: &FaqItemContent,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_item_content::create(tx, item_content.into())?;
        Ok(())
    }

    fn next_category_item_display_order(
        &self,
        tx: &mut Self::Transaction,
        faq_category_id: &FaqCategoryId,
    ) -> Result<u32, Self::Err> {
        let current_max =
            database::adapters::faq_category_item::get_max_display_order_by_faq_category_id(
                tx,
                &faq_category_id,
            )?
            .unwrap_or(0);
        Ok(current_max + 1)
    }

    fn create_category_item(
        &self,
        tx: &mut Self::Transaction,
        category_item: &FaqCategoryItem,
    ) -> Result<(), Self::Err> {
        database::adapters::faq_category_item::create(tx, category_item.into())?;
        Ok(())
    }
}
