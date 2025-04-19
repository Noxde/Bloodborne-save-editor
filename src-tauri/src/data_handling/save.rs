use serde::{Deserialize, Serialize};

use std::path::PathBuf;

use super::{
    article::Article, bosses::{self, Boss}, enums::{ArticleType, Error, Location, UpgradeType}, file::FileData, inventory::Inventory, slots::{parse_equipped_gems, Slot}, stats::{self, Stat}, upgrades::{parse_upgrades, Upgrade}, username::Username
};

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    #[serde(skip_serializing)]
    pub file: FileData,
    pub stats: Vec<Stat>,
    pub inventory: Inventory,
    pub storage: Inventory,
    pub username: Username,
    pub bosses: Vec<Boss>,
    pub playtime: u32
}

impl SaveData {
    pub fn build(save_path: &str, resources_path: PathBuf) -> Result<SaveData, Error> {
        let mut file = FileData::build(save_path, resources_path)?;
        let stats = stats::new(&file).unwrap();
        let bosses = bosses::new(&file).unwrap();
        let mut upgrades = parse_upgrades(&file);
        let mut slots = parse_equipped_gems(&mut file, &mut upgrades);
        let inventory = Inventory::build(&file, file.offsets.inventory, file.offsets.key_inventory, &mut upgrades, &mut slots);
        let storage = Inventory::build(&file, file.offsets.storage, (0,0), &mut upgrades, &mut slots); // Its not possible to store key items
        let username = Username::build(&file);
        let playtime = file.get_playtime();

        Ok(SaveData {
            file,
            stats,
            inventory,
            storage,
            username,
            bosses,
            playtime
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

    pub fn get_upgrade_mut(&mut self, location: Location, upgrade_type: UpgradeType, upgrade_index: usize) -> Option<&mut Upgrade> {
        let upgrades = match location {
            Location::Inventory => &mut self.inventory.upgrades,
            Location::Storage => &mut self.storage.upgrades,
        };

        if let Some(upgrades_of_type) = upgrades.get_mut(&upgrade_type) {
            return upgrades_of_type.get_mut(upgrade_index);
        }
        None
    }

    pub fn move_upgrade(&mut self, upgrade_type: UpgradeType, upgrade_index: usize, from: Location) -> Result<(), Error> {
        match from {
            Location::Inventory => {
                let upgrade = self.inventory.remove_upgrade(&mut self.file, upgrade_type, upgrade_index, false)?;
                self.storage.add_upgrade(&mut self.file, upgrade, true);

            },
            Location::Storage => {
                let upgrade = self.storage.remove_upgrade(&mut self.file, upgrade_type, upgrade_index, true)?;
                self.inventory.add_upgrade(&mut self.file, upgrade, false);

            },
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf,
              time::Instant};

    use super::*;
    use crate::data_handling::{enums::SlotShape,
                               utils::test_utils::{build_save_data, check_bytes}};

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

    #[test]
    fn test_get_upgrade_mut() {
        //Inventory
        let mut save = SaveData::build("saves/testsave5", PathBuf::from("resources")).unwrap();
        let upgrades = save.inventory.upgrades.clone();
        let upgrades_of_type = upgrades.get(&UpgradeType::Rune).unwrap();
        let upgrade1 = upgrades_of_type.get(0).unwrap();
        let upgrade2 = save.get_upgrade_mut(Location::Inventory, UpgradeType::Rune, 0).unwrap();
        assert_eq!(*upgrade1, *upgrade2);
        assert_eq!(upgrade1.id, 3229614361);

        upgrade2.id = 0;

        let upgrades = save.inventory.upgrades.clone();
        let upgrades_of_type = upgrades.get(&UpgradeType::Rune).unwrap();
        let upgrade1 = upgrades_of_type.get(0).unwrap();
        assert_eq!(upgrade1.id, 0);

        //Storage
        let mut save = SaveData::build("saves/testsave9", PathBuf::from("resources")).unwrap();
        let upgrades = save.storage.upgrades.clone();
        let upgrades_of_type = upgrades.get(&UpgradeType::Gem).unwrap();
        let upgrade1 = upgrades_of_type.get(0).unwrap();
        let upgrade2 = save.get_upgrade_mut(Location::Storage, UpgradeType::Gem, 0).unwrap();
        assert_eq!(*upgrade1, *upgrade2);
        assert_eq!(upgrade1.id, 3229614193);

        upgrade2.id = 0;

        let upgrades = save.storage.upgrades.clone();
        let upgrades_of_type = upgrades.get(&UpgradeType::Gem).unwrap();
        let upgrade1 = upgrades_of_type.get(0).unwrap();
        assert_eq!(upgrade1.id, 0);

        //Not found
        assert!(save.get_upgrade_mut(Location::Storage, UpgradeType::Rune, 0).is_none());
        assert!(save.get_upgrade_mut(Location::Storage, UpgradeType::Gem, usize::MAX).is_none());
    }

    #[test]
    #[ignore] //cargo test -- --include-ignored
    fn test_save_data_get_muts_runtime() {
        let mut save = SaveData::build("saves/testsave5", PathBuf::from("resources")).unwrap();

        //Test get_slot_mut() runtime
        let now = Instant::now();
        save.get_slot_mut(Location::Inventory, ArticleType::RightHand, 0, 0).unwrap();
        let elapsed = now.elapsed().as_micros();
        assert!(elapsed < 10);

        //Test get_article_mut() runtime
        let now = Instant::now();
        save.get_article_mut(Location::Inventory, ArticleType::RightHand, 0).unwrap();
        let elapsed = now.elapsed().as_micros();
        assert!(elapsed < 10);

        //Test get_equipped_upgrade_mut() runtime
        let now = Instant::now();
        save.get_equipped_upgrade_mut(Location::Inventory, ArticleType::RightHand, 0, 3).unwrap();
        let elapsed = now.elapsed().as_micros();
        assert!(elapsed < 10);

        //Test get_upgrade_mut() runtime
        let now = Instant::now();
        save.get_upgrade_mut(Location::Inventory, UpgradeType::Rune, 0).unwrap();
        let elapsed = now.elapsed().as_micros();
        assert!(elapsed < 10);
    }

    #[test]
    fn test_move_upgrade() {
        let mut save = build_save_data("testsave9");

        //Test error cases
        let result = save.move_upgrade(UpgradeType::Rune, 500, Location::Storage);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: There are no upgrades of the specified type.");
        }

        let result = save.move_upgrade(UpgradeType::Gem, 500, Location::Storage);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.to_string(), "Save error: ERROR: upgrade_index is invalid.");
        }

        //The inventory has 2 gems
        assert_eq!(save.inventory.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 2);
        //The storage has 2 gems
        assert_eq!(save.storage.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 2);
        //Get the gem to be moved
        let gem = save.inventory.upgrades.get_mut(&UpgradeType::Gem).unwrap()[0].clone();
        //Slot of the inventory with the gem
        assert!(check_bytes(&save.file, 0x8fe8,
            &[0x51,0x40,0x89,0x13,0x73,0x00,0x80,0xc0,0xf0,0x49,0x02,0x80,0x01,0x00,0x00,0x00]));
        //Last slot of the storage
        assert!(check_bytes(&save.file, 0x11524,
            &[0x69,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0x00,0x00,0x00,0x00]));

        //Move the gem
        save.move_upgrade(UpgradeType::Gem, 0, Location::Inventory).unwrap();

        //The inventory now has 1 gem
        assert_eq!(save.inventory.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 1);
        //The storage now has 3 gems
        assert_eq!(save.storage.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 3);
        //Get the moved gem
        let mut moved_gem = save.storage.upgrades.get_mut(&UpgradeType::Gem).unwrap().last_mut().unwrap().clone();
        assert_eq!(moved_gem.index, 2);
        moved_gem.index = 0;
        assert_eq!(gem, moved_gem);
        //Now the inventory slot in which the gem was is empty
        assert!(check_bytes(&save.file, 0x8fe8,
            &[0x51,0x40,0x89,0x13,0,0,0,0,255,255,255,255,0,0,0,0]));
        //And the last slot of the storage has the gem
        assert!(check_bytes(&save.file, 0x11524,
            &[0x69,0x00,0x00,0x00,0x73,0x00,0x80,0xc0,0xf0,0x49,0x02,0x80,0x01,0x00,0x00,0x00,
              0x6a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xff,0xff,0xff,0xff,0x00,0x00,0x00,0x00]));

        //Try again but from the storage

        //The inventory has 1 gems
        assert_eq!(save.inventory.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 1);
        //The storage has 3 gems
        assert_eq!(save.storage.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 3);
        //Get the gem to be moved
        let gem = save.storage.upgrades.get_mut(&UpgradeType::Gem).unwrap()[0].clone();
        //Slot of the storage with the gem
        assert!(check_bytes(&save.file, 0x11504,
            &[0x44,0x40,0x89,0x13,0x71,0x00,0x80,0xc0,0x48,0xe8,0x01,0x80,0x01,0x00,0x00,0x00]));
        //Empty slot of the inventory
        assert!(check_bytes(&save.file, 0x8fe8,
            &[0x51,0x40,0x89,0x13,0,0,0,0,255,255,255,255,0,0,0,0]));

        //Move the gem
        save.move_upgrade(UpgradeType::Gem, 0, Location::Storage).unwrap();

        //The inventory now has 2 gem
        assert_eq!(save.inventory.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 2);
        //The storage now has 2 gems
        assert_eq!(save.storage.upgrades.get_mut(&UpgradeType::Gem).unwrap().len(), 2);
        //Get the moved gem
        let mut moved_gem = save.inventory.upgrades.get_mut(&UpgradeType::Gem).unwrap().last_mut().unwrap().clone();
        assert_eq!(moved_gem.index, 1);
        moved_gem.index = 0;
        assert_eq!(gem, moved_gem);
        //Now the storage slot in which the gem was is empty
        assert!(check_bytes(&save.file, 0x11504,
            &[0x44,0x40,0x89,0x13,0,0,0,0,255,255,255,255,0,0,0,0]));
        //The slot now has the gem
        assert!(check_bytes(&save.file, 0x8fe8,
            &[0x51,0x40,0x89,0x13,0x71,0x00,0x80,0xc0,0x48,0xe8,0x01,0x80,0x01,0x00,0x00,0x00]));
    }
}
