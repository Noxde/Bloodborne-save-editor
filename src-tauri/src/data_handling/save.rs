use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use super::{
    enums::{Error, ArticleType, Location},
    file::FileData,
    inventory::{Inventory, Article},
    stats::{self, Stat},
    upgrades::{parse_upgrades, Upgrade},
    username::Username,
    slots::{parse_equipped_gems, Slot},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub file: FileData,
    pub stats: Vec<Stat>,
    pub inventory: Inventory,
    pub storage: Inventory,
    pub username: Username,
}

impl SaveData {
    pub fn build(save_path: &str, resources_path: PathBuf) -> Result<SaveData, Error> {
        let mut file = FileData::build(save_path, resources_path)?;
        let stats = stats::new(&file).unwrap();
        let mut upgrades = parse_upgrades(&file);
        let mut slots = parse_equipped_gems(&mut file, &mut upgrades);
        let inventory = Inventory::build(&file, file.offsets.inventory, file.offsets.key_inventory, &mut upgrades, &mut slots);
        let storage = Inventory::build(&file, file.offsets.storage, (0,0), &mut upgrades, &mut slots); // Its not possible to store key items
        let username = Username::build(&file);

        Ok(SaveData {
            file,
            stats,
            inventory,
            storage,
            username,
        })
    }

    pub fn get_slot_mut(&mut self, location: Location, article_type: ArticleType, article_index: usize, slot_index: usize) -> Option<&mut Slot> {
        let articles = match location {
            Location::Inventory => &mut self.inventory.articles,
            Location::Storage => &mut self.storage.articles,
        };

        if let Some(articles_of_type) = articles.get_mut(&article_type) {
            if let Some(article) = articles_of_type.get_mut(article_index) {
                if let Some(ref mut slots) = &mut article.slots {
                    return slots.get_mut(slot_index);
                }
            }
        }
        None
    }

    pub fn get_article_mut(&mut self, location: Location, article_type: ArticleType, article_index: usize) -> Option<&mut Article> {
        let articles = match location {
            Location::Inventory => &mut self.inventory.articles,
            Location::Storage => &mut self.storage.articles,
        };

        if let Some(articles_of_type) = articles.get_mut(&article_type) {
            return articles_of_type.get_mut(article_index);
        }
        None
    }

    pub fn get_equipped_upgrade_mut(&mut self, location: Location, article_type: ArticleType, article_index: usize, slot_index: usize) -> Option<&mut Upgrade> {
        if let Some(slot) = self.get_slot_mut(location, article_type, article_index, slot_index) {
            if let Some(ref mut gem) = &mut slot.gem {
                return Some(gem);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::usize;

    use super::*;
    use crate::data_handling::enums::SlotShape;

    #[test]
    fn test_build() {
        assert!(SaveData::build("saves/testsave0", PathBuf::from("resources")).is_ok());
    }

    #[test]
    fn test_get_slot_mut() {
        //Inventory
        let mut save = SaveData::build("saves/testsave5", PathBuf::from("resources")).unwrap();
        let articles = save.inventory.articles.clone();
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article = articles_of_type.get(0).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot1 = slots.get(0).unwrap();
        let slot2 = save.get_slot_mut(Location::Inventory, ArticleType::RightHand, 0, 0).unwrap();
        assert_eq!(*slot1, *slot2);
        assert_eq!(slot1.shape, SlotShape::Droplet);

        slot2.shape = SlotShape::Triangle;

        let articles = save.inventory.articles;
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article = articles_of_type.get(0).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot1 = slots.get(0).unwrap();
        assert_eq!(slot1.shape, SlotShape::Triangle);


        //Storage
        let mut save = SaveData::build("saves/testsave5", PathBuf::from("resources")).unwrap();
        let articles = save.storage.articles.clone();
        let articles_of_type = articles.get(&ArticleType::Armor).unwrap();
        let article = articles_of_type.get(0).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot1 = slots.get(0).unwrap();
        let slot2 = save.get_slot_mut(Location::Storage, ArticleType::Armor, 0, 0).unwrap();
        assert_eq!(*slot1, *slot2);
        assert_eq!(slot1.shape, SlotShape::Closed);

        slot2.shape = SlotShape::Waning;

        let articles = save.storage.articles.clone();
        let articles_of_type = articles.get(&ArticleType::Armor).unwrap();
        let article = articles_of_type.get(0).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot1 = slots.get(0).unwrap();
        assert_eq!(slot1.shape, SlotShape::Waning);

        //Not found
        assert!(save.get_slot_mut(Location::Storage, ArticleType::Chalice, 0, 0).is_none());
        assert!(save.get_slot_mut(Location::Storage, ArticleType::Armor, usize::MAX, 0).is_none());
        assert!(save.get_slot_mut(Location::Storage, ArticleType::Armor, 0, usize::MAX).is_none());
    }

    #[test]
    fn test_get_article_mut() {
        //Inventory
        let mut save = SaveData::build("saves/testsave5", PathBuf::from("resources")).unwrap();
        let articles = save.inventory.articles.clone();
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article1 = articles_of_type.get(0).unwrap();
        let article2 = save.get_article_mut(Location::Inventory, ArticleType::RightHand, 0).unwrap();
        assert_eq!(*article1, *article2);
        assert_eq!(article1.id, 28001000);

        article2.id = 0;

        let articles = save.inventory.articles.clone();
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article1 = articles_of_type.get(0).unwrap();
        assert_eq!(article1.id, 0);

        //Storage
        let mut save = SaveData::build("saves/testsave5", PathBuf::from("resources")).unwrap();
        let articles = save.storage.articles.clone();
        let articles_of_type = articles.get(&ArticleType::Armor).unwrap();
        let article1 = articles_of_type.get(0).unwrap();
        let article2 = save.get_article_mut(Location::Storage, ArticleType::Armor, 0).unwrap();
        assert_eq!(*article1, *article2);
        assert_eq!(article1.id, 351000);

        article2.id = 0;

        let articles = save.storage.articles.clone();
        let articles_of_type = articles.get(&ArticleType::Armor).unwrap();
        let article1 = articles_of_type.get(0).unwrap();
        assert_eq!(article1.id, 0);

        //Not found
        assert!(save.get_article_mut(Location::Storage, ArticleType::Chalice, 0).is_none());
        assert!(save.get_article_mut(Location::Storage, ArticleType::Armor, usize::MAX).is_none());
    }

    #[test]
    fn test_get_equipped_upgrade_mut() {
        //Inventory
        let mut save = SaveData::build("saves/testsave8", PathBuf::from("resources")).unwrap();
        let articles = save.inventory.articles.clone();
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article = articles_of_type.get(0).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot = slots.get(0).unwrap();
        let gem1 = slot.gem.as_ref().unwrap();
        let gem2 = save.get_equipped_upgrade_mut(Location::Inventory, ArticleType::RightHand, 0, 0).unwrap();
        assert_eq!(*gem1, *gem2);
        assert_eq!(gem1.id, 3229615259);

        gem2.id = 0;

        let articles = save.inventory.articles;
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article = articles_of_type.get(0).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot = slots.get(0).unwrap();
        let gem1 = slot.gem.as_ref().unwrap();
        assert_eq!(gem1.id, 0);

        //Storage
        let mut save = SaveData::build("saves/testsave8", PathBuf::from("resources")).unwrap();
        let articles = save.storage.articles.clone();
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article = articles_of_type.get(17).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot = slots.get(0).unwrap();
        let gem1 = slot.gem.as_ref().unwrap();
        let gem2 = save.get_equipped_upgrade_mut(Location::Storage, ArticleType::RightHand, 17, 0).unwrap();
        assert_eq!(*gem1, *gem2);
        assert_eq!(gem1.id, 3229614569);

        gem2.id = 0;

        let articles = save.storage.articles.clone();
        let articles_of_type = articles.get(&ArticleType::RightHand).unwrap();
        let article = articles_of_type.get(17).unwrap();
        let slots = &article.slots.as_ref().unwrap();
        let slot = slots.get(0).unwrap();
        let gem1 = slot.gem.as_ref().unwrap();
        assert_eq!(gem1.id, 0);

        //Not found
        assert!(save.get_equipped_upgrade_mut(Location::Storage, ArticleType::Chalice, 0, 0).is_none());
        assert!(save.get_equipped_upgrade_mut(Location::Storage, ArticleType::Armor, usize::MAX, 0).is_none());
        assert!(save.get_equipped_upgrade_mut(Location::Storage, ArticleType::Armor, 0, usize::MAX).is_none());
    }
}
