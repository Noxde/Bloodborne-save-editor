//Distance between the username and the beginning of the inventory
pub const USERNAME_TO_INV_OFFSET: usize = 469;

//Distance between the username and the beginning of the key items inventory
pub const USERNAME_TO_KEY_INV_OFFSET: usize = 32201;

//Distance between the username and the beginning of the storage inventory
pub const INV_TO_STORAGE_OFFSET: usize = 34268;

// Distance between the username and the first inventory counter
pub const USERNAME_TO_FIRST_INVENTORY_COUNTER: usize = 453;

// Distance between the username and the second inventory counter
pub const USERNAME_TO_SECOND_INVENTORY_COUNTER: usize = 34257;

// Distance between the username and the first inventory counter
pub const USERNAME_TO_FIRST_STORAGE_COUNTER: usize = 34737;

// Distance between the username and the second inventory counter
pub const USERNAME_TO_SECOND_STORAGE_COUNTER: usize = 68541;

// Distance between the username and the isz glitch bytes
pub const USERNAME_TO_ISZ_GLITCH: usize = 72082;

//Distance from the start of the save to the first gem/rune
pub const START_TO_UPGRADE: usize = 84;

//Amount of bytes used to store the character appearance
pub const APPEARANCE_BYTES_AMOUNT: usize = 0xEB;

//Amount of empty slots that can be detected while parsing the inventory before considering it finished
pub const MAX_EMPTY_INV_SLOTS: usize = 20;
