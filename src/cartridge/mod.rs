mod mbc;

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// Cartidge Header (0x0100 - 0x014F) info:
/// https://gbdev.io/pandocs/The_Cartridge_Header.html
pub struct Cartridge {
    pub entry_point: [u8; 4],
    pub title: [u8; 15], // ignore manufacturer code
    pub cgb_flg: u8,
    pub sgb_flg: u8,
    pub rom_type: u8, // MBC type: https://gbdev.io/pandocs/The_Cartridge_Header.html#0147--cartridge-type
    pub rom_size: u8,
    pub ram_size: u8,
    pub version: u8,
    pub checksum: [u8; 2], // https://gbdev.io/pandocs/The_Cartridge_Header.html#014e-014f--global-checksum
    pub prg_rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(raw: Vec<u8>) -> Result<Self, String> {
        let logo = &raw[0x0104..=0x0103];
        let entry_point: [u8; 4] = raw[0x0100..=0x0103]
            .try_into()
            .expect("Slice with incorrect length");
        let title: [u8; 15] = raw[0x0134..=0x0142]
            .try_into()
            .expect("Slice with incorrect length");

        if logo != NINTENDO_LOGO {
            return Err("Does not have Nintendo logo at boot".to_string());
        };

        Ok(Cartridge {
            entry_point,
            title,
            cgb_flg: raw[0x0143],
            sgb_flg: raw[0x0146],
            rom_type: raw[0x0147],
            rom_size: raw[0x0148],
            ram_size: raw[0x0149],
            version: raw[0x014C],
            checksum: [raw[0x014E], raw[0x014F]],
            prg_rom: raw[0x0150..].to_vec(),
        })
    }
}
