//! Original native-density demo pixel art. No reference atlas extraction or simulation mutation.
use retrosim_sim::{Action, Actor, EffectKind, IntentKind, Object, Role, State, Team};

pub const WIDTH: usize = 440;
pub const HEIGHT: usize = 240;
const TILE: i32 = 20;
const INK: u32 = 0x102a32;
const MOSS: u32 = 0x75935a;
const PATH: u32 = 0xb18b59;
const STONE: u32 = 0x7c9086;
const STONE_LIGHT: u32 = 0xb4bca1;
const STONE_DARK: u32 = 0x526961;
const GOLD: u32 = 0xf0cf79;
const CYAN: u32 = 0x74ece0;
const RED: u32 = 0xed8b76;
const WHITE: u32 = 0xf3edcd;

pub struct View {
    pub selected: usize,
    pub hover: Option<(i32, i32)>,
    pub selected_action: &'static str,
    pub queued: Option<Action>,
    pub grid: bool,
    /// Elapsed seconds, used only for restrained visual animation.
    pub time: f32,
    pub since_beat: f32,
}

struct Painter {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    alpha: u8,
}
impl Painter {
    fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0; width * height * 4],
            width,
            height,
            alpha: 255,
        }
    }
    fn rect(&mut self, color: u32, x: i32, y: i32, width: i32, height: i32) {
        let rgb = [(color >> 16) as u8, (color >> 8) as u8, color as u8];
        let alpha = self.alpha as u32;
        for py in y.max(0)..(y + height).min(self.height as i32) {
            for px in x.max(0)..(x + width).min(self.width as i32) {
                let index = (py as usize * self.width + px as usize) * 4;
                for (channel, value) in rgb.iter().enumerate() {
                    self.pixels[index + channel] = ((u32::from(*value) * alpha
                        + u32::from(self.pixels[index + channel]) * (255 - alpha))
                        / 255) as u8;
                }
                self.pixels[index + 3] = 255;
            }
        }
    }
}
fn palette(pixel: u8) -> Option<u32> {
    Some(match pixel {
        b'k' => INK,
        b'h' => 0xd8aa53,
        b'H' => 0xf7d778,
        b'y' => 0xa87537,
        b's' => 0xe8b788,
        b'S' => 0xf7d0a0,
        b'b' => 0x82583d,
        b'B' => 0xb57946,
        b't' => 0x267f7d,
        b'T' => 0x55b7a5,
        b'u' => 0x195b63,
        b'g' => 0x426f49,
        b'G' => 0x89a65d,
        b'l' => 0xc6d2c0,
        b'L' => 0xedf1d5,
        b'a' => 0x839b9c,
        b'A' => 0xc2cdba,
        b'd' => 0x526976,
        b'w' => 0xeadbb6,
        b'r' => 0xad5145,
        b'R' => 0xdf8361,
        b'p' => 0x69545b,
        b'P' => 0x957079,
        b'o' => 0xb17b50,
        b'O' => 0xe0a263,
        b'e' => 0x94e2dc,
        b'v' => 0x333c47,
        b'f' => 0x5a7463,
        _ => return None,
    })
}
const PLAYER: &[&str] = &[
    "......HHH.........",
    "....HHHHhHh.......",
    "...hHHHhHhhh......",
    "...hHhhhSSSh......",
    "...hSSSSSSSS......",
    "...skkekkekS......",
    "...SkeekkekS......",
    "....SSSSSSS.......",
    ".....hBhhS........",
    "....tThhTt.....L..",
    "...TTTwwTTt...LA..",
    "..STTtuutTTS..A...",
    "..SSttuuuttS..A...",
    "...StuTTut.S.b....",
    "....tTyyt..bBw....",
    "....TBHyT...b.....",
    "....ttutt.........",
    "....TTuTT.........",
    "....bb.bb.........",
    "....Bb.Bb.........",
    "...BBB.BBB........",
    "...bbb.bbb........",
];

const SCOUT: &[&str] = &[
    "......LLl.........",
    ".....LLLLl........",
    "....lLLLLll.......",
    "....lLSSSlL.......",
    "....LSkSSkL.......",
    "....LSSSSSL.......",
    "...lLLSSSLL.......",
    "...lLgRRgLl.......",
    "....gGRRgG........",
    "...gGGwwGGg...b...",
    "..gGgggGgGg...Bb..",
    "..SGgggggGS....b..",
    "..SSgGbggSS...b...",
    "...SggBBgS....b...",
    "....gGHgg....Bb...",
    "....GGgGG...b.....",
    "....GGgGG.........",
    "....bg.gb.........",
    "....Bb.bB.........",
    "....Bb.bB.........",
    "...BBB.BBB........",
    "...bbb.bbb........",
];

const WARDEN: &[&str] = &[
    ".......H..........",
    ".....AAHaa........",
    "....AAAHaaa.......",
    "....aAAHaaA.......",
    "....aHHHHHa.......",
    "....SkSSkSS.......",
    "....SSOSSSS.......",
    ".....OOOOO........",
    "....OOBOOO....A...",
    "....aOOOOa...AAA..",
    "..aaAaBOaAa...a...",
    ".aAHHAagaaAS..b...",
    ".aAHHAAgaaSS..b...",
    ".aAHHAAgaaS...b...",
    " .aaHAagya...b....",
    "..aaaaBHaa...b....",
    "....gagaag........",
    "....gagagg........",
    "....bb.bb.........",
    "....BB.BB.........",
    "...BBB.BBB........",
    "...bbb.bbb........",
];

const RAIDER: &[&str] = &[
    "......ppp.........",
    ".....pPPPp........",
    "....pPPPPpp.......",
    "....pPSSSpP.......",
    "....SSkSSkS.......",
    "....SSSSSSS.......",
    ".....SSbSS........",
    "....RRRRRR........",
    "...pPRrrPPp.......",
    "..pPPPrPPPPS...A..",
    "..SPPpppPPS...AA..",
    "..SSPpppPSS....A..",
    "...SPpppPS.....b..",
    "....pByBp....bBw..",
    "....ppHpp.....b...",
    "....PPpPP.........",
    "....pp.pp.........",
    "....bb.bb.........",
    "....Bb.bB.........",
    "...BBB.BBB........",
    "...bbb.bbb........",
];

const ARCHER: &[&str] = &[
    "......rrrr........",
    ".....rRRRrr.......",
    "....rRRRRRr.......",
    "....rRSSSrr.......",
    "....rSkSSkr.......",
    "....rSSSSSr.......",
    ".....rSSSr........",
    "....rRRRRrr.......",
    "...rRvvvvRrr..b...",
    "..rRvAvvvvRS..Bb..",
    "..SRvvvvvvS....b..",
    "..SSvvvvvSS...b...",
    "...SvBvvvS....b...",
    "....vBBvv....Bb...",
    "....vvHvv...b.....",
    "....vvvvd.........",
    "....dv.dv.........",
    "....bb.bb.........",
    "....Bb.bB.........",
    "...BBB.BBB........",
    "...bbb.bbb........",
];

const BRUTE: &[&str] = &[
    ".....yyyyyy.......",
    "....yhhhhhhy......",
    "...yhSSSSSSHy.....",
    "...SSSkSSkSSS.....",
    "...SSSSSSSSSS.....",
    "....SSBSSBSS......",
    "....rRRRRRrr......",
    "..ppRRRrrrPPpp....",
    ".pPPPrPPPPPPPPp...",
    ".SPPPPppppPPPPSS..",
    ".SSPPPppppPPPSSS..",
    ".SSSppppppppSSS...",
    "..SSpBBBBppSS.....",
    "...ppByHBpp....bb.",
    "...pppppppp...BBB.",
    "...PPppppPP...BBB.",
    "...PPp..pPP...bb..",
    "...bbb..bbb.......",
    "...BBb..bBB.......",
    "..BBBB..BBBB......",
    "..bbbb..bbbb......",
];

fn rows(role: Role) -> &'static [&'static str] {
    match role {
        Role::Player => PLAYER,
        Role::Scout => SCOUT,
        Role::Warden => WARDEN,
        Role::Raider => RAIDER,
        Role::Archer => ARCHER,
        Role::Brute => BRUTE,
    }
}
fn sprite(p: &mut Painter, role: Role, x: i32, y: i32) {
    let rows = rows(role);
    let occupied = |px: i32, py: i32| -> bool {
        if px < 0 || py < 0 {
            return false;
        }
        rows.get(py as usize)
            .and_then(|r| r.as_bytes().get(px as usize))
            .and_then(|v| palette(*v))
            .is_some()
    };
    for (sy, row) in rows.iter().enumerate() {
        for (sx, v) in row.bytes().enumerate() {
            if palette(v).is_some() {
                let (sx, sy) = (sx as i32, sy as i32);
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if !occupied(sx + dx, sy + dy) {
                        p.rect(INK, x + sx + dx, y + sy + dy, 1, 1);
                    }
                }
            }
        }
    }
    for (sy, row) in rows.iter().enumerate() {
        for (sx, v) in row.bytes().enumerate() {
            if let Some(color) = palette(v) {
                p.rect(color, x + sx as i32, y + sy as i32, 1, 1);
            }
        }
    }
}
fn hash(x: i32, y: i32, salt: u32) -> u32 {
    let mut n = (x as u32).wrapping_add(37).wrapping_mul(374_761_393)
        ^ (y as u32).wrapping_add(71).wrapping_mul(668_265_263)
        ^ salt;
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    n ^ (n >> 16)
}
fn tile(state: &State, x: i32, y: i32) -> char {
    if x < 0 || y < 0 {
        return '#';
    }
    state
        .tiles
        .get(y as usize)
        .and_then(|r| r.get(x as usize))
        .copied()
        .unwrap_or('#')
}
fn blocked(t: char) -> bool {
    matches!(t, '#' | 'R' | '~')
}
fn ground(p: &mut Painter, state: &State, x: i32, y: i32, time: f32) {
    let t = tile(state, x, y);
    let (px, py) = (x * TILE, y * TILE);
    let n = hash(x, y, 0);
    let odd = n & 1 != 0;
    p.rect(
        match t {
            ':' => {
                if odd {
                    PATH
                } else {
                    0xaf8755
                }
            }
            '=' => STONE_DARK,
            '~' => {
                if odd {
                    0x236f77
                } else {
                    0x1c5965
                }
            }
            '#' => 0x224b3d,
            _ => {
                if odd {
                    0x386748
                } else {
                    0x3b6b4b
                }
            }
        },
        px,
        py,
        TILE,
        TILE,
    );
    if matches!(t, '.' | ':') {
        if t == ':' {
            if tile(state, x - 1, y) == '.' {
                p.rect(0x6d7748, px, py, 2, 20);
                p.rect(PATH, px + 1, py + 4, 2, 10);
            }
            if tile(state, x + 1, y) == '.' {
                p.rect(0x6d7748, px + 18, py, 2, 20);
                p.rect(PATH, px + 17, py + 7, 2, 10);
            }
            if tile(state, x, y - 1) == '.' {
                p.rect(0x6d7748, px + 3, py, 13, 1);
            }
            if tile(state, x, y + 1) == '.' {
                p.rect(0x6d7748, px + 5, py + 19, 12, 1);
            }
        }
        for i in 0..5 {
            let h = hash(x, y, i * 11779 + 3);
            let color = if t == ':' {
                if i % 2 == 1 { 0x99794e } else { 0xc4a16a }
            } else if i % 2 == 1 {
                0x315e46
            } else {
                0x4c7b51
            };
            p.rect(
                color,
                px + 2 + (h % 15) as i32,
                py + 2 + ((h >> 8) % 15) as i32,
                (i % 2 + 1) as i32,
                1,
            );
        }
        if t == '.' && n % 5 == 0 {
            let gx = px + 4 + ((n >> 4) % 10) as i32;
            let gy = py + 9 + ((n >> 11) % 6) as i32;
            p.rect(0x567d50, gx, gy - 3, 1, 4);
            p.rect(MOSS, gx + 2, gy - 5, 1, 4);
            p.rect(0x567d50, gx + 4, gy - 2, 1, 3);
        }
        if t == '.' && n % 19 == 0 {
            p.rect(0xb4bb79, px + 13, py + 8, 2, 2);
            p.rect(0x5b8a62, px + 13, py + 10, 1, 2);
        }
    } else if t == '=' {
        p.rect(STONE, px + 1, py + 1, 18, 17);
        p.rect(0x96a193, px + 2, py + 1, 16, 1);
        p.rect(0x697e72, px + 1, py + 17, 18, 2);
        p.rect(MOSS, px + if odd { 1 } else { 15 }, py + 13, 4, 5);
        p.rect(STONE_DARK, px + 6 + (n % 5) as i32, py + 6, 2, 3);
        p.rect(0xa5ad96, px + 4, py + 4, 3, 2);
    } else if t == '~' {
        p.rect(0x297e80, px + 3, py + 3, 8, 2);
        p.rect(0x236874, px + 10, py + 13, 8, 2);
        if n % 3 == 0 {
            let shift = ((time / 0.6).floor() as u32 + n % 3) % 3;
            p.rect(0x4daca5, px + 3 + shift as i32, py + 8, 6, 1);
            p.rect(0x81c3b3, px + 5 + shift as i32, py + 7, 3, 1);
        }
        if tile(state, x - 1, y) != '~' {
            p.rect(0x548366, px, py + 2, 2, 16);
        }
        if tile(state, x, y - 1) != '~' {
            p.rect(0x548366, px + 1, py, 18, 2);
        }
    }
}
fn tree(p: &mut Painter, x: i32, y: i32) {
    let (px, py) = (x * TILE, y * TILE);
    let n = hash(x, y, 0);
    let shift = (n % 3) as i32 - 1;
    p.rect(0x173c36, px + 1, py + 12, 19, 8);
    p.rect(0x73553b, px + 8, py + 8, 5, 11);
    p.rect(0x9b7447, px + 8, py + 11, 2, 7);
    p.rect(0x263f35, px + 5, py + 18, 11, 2);
    p.rect(0x173d36, px - 1, py + 4, 22, 10);
    p.rect(0x245342, px + 1, py, 19, 13);
    p.rect(0x326347, px + 3 + shift, py - 3, 14, 14);
    p.rect(0x326347, px, py + 3, 6, 7);
    p.rect(0x427951, px + 3 + shift, py + 1, 6, 5);
    p.rect(0x538456, px + 7 + shift, py - 2, 7, 5);
    p.rect(0x668f58, px + 6 + shift, py, 4, 2);
    p.rect(0x3d7150, px + 12, py + 4, 7, 5);
    p.rect(0x2c5944, px + 3, py + 9, 9, 5);
    p.rect(0x204d40, px + 11, py + 12, 6, 3);
    if n % 2 == 1 {
        p.rect(0x4a7b4e, px + 14, py + 1, 3, 3);
        p.rect(0x2a5a41, px + 4, py + 6, 3, 3);
    } else {
        p.rect(0x608c56, px + 4, py + 3, 2, 2);
        p.rect(0x2a5a41, px + 13, py + 8, 3, 3);
    }
}
fn rubble(p: &mut Painter, x: i32, y: i32) {
    let (px, py) = (x * TILE, y * TILE);
    p.rect(0x284c40, px + 1, py + 15, 19, 5);
    p.rect(INK, px + 3, py + 7, 14, 11);
    p.rect(STONE_DARK, px + 2, py + 9, 16, 7);
    p.rect(STONE, px + 4, py + 4, 12, 11);
    p.rect(0x9baa95, px + 6, py + 2, 8, 4);
    p.rect(STONE_LIGHT, px + 5, py + 4, 6, 2);
    p.rect(0x526d64, px + 12, py + 6, 4, 8);
    p.rect(0xb2b8a0, px + 2, py + 11, 4, 2);
    p.rect(0x769653, px + 5, py + 14, 7, 3);
    p.rect(MOSS, px + 12, py + 10, 4, 3);
    p.rect(0x344f44, px + 9, py + 8, 2, 4);
}
fn masonry(p: &mut Painter, x: i32, y: i32, state: &State) {
    let (px, py) = (x * TILE, y * TILE);
    let pillar = tile(state, x, y + 1) == 'R' || tile(state, x, y - 1) == 'R';
    p.rect(0x25483e, px, py + 16, 20, 4);
    if !pillar && y >= 4 {
        rubble(p, x, y);
        return;
    }
    let (left, width) = if pillar { (3, 14) } else { (0, 20) };
    p.rect(INK, px + left - 1, py - 3, width + 2, 22);
    p.rect(STONE_DARK, px + left, py - 3, width, 21);
    p.rect(STONE, px + left, py - 3, width - 3, 18);
    p.rect(0xa9b59c, px + left, py - 3, width - 2, 2);
    p.rect(0x8e9e8d, px + left + 1, py, 2, 14);
    p.rect(0x495f58, px + left, py + 5, width - 1, 1);
    p.rect(0xa5af97, px + left, py + 6, width - 4, 1);
    p.rect(0x495f58, px + left, py + 12, width - 1, 1);
    p.rect(0x3e594e, px + left + 8, py - 1, 1, 6);
    p.rect(0x3e594e, px + left + 5, py + 7, 1, 5);
    p.rect(0x3e594e, px + left + 9, py + 13, 1, 3);
    p.rect(0x729054, px + left + width - 5, py - 3, 5, 4);
    p.rect(0x98a85f, px + left + width - 3, py - 2, 2, 1);
    p.rect(0x668350, px + left, py + 14, 5, 4);
    if y == 1 {
        p.rect(STONE_DARK, px + left - 1, py - 6, width + 2, 3);
        p.rect(0xadb59d, px + left, py - 6, width, 1);
    }
}
fn marker(p: &mut Painter, obj: &Object, time: f32) {
    let (x, y) = (obj.x * TILE + 3, obj.y * TILE - 5);
    p.rect(0x284e48, x - 2, y + 19, 18, 6);
    p.rect(0x254e50, x, y + 17, 14, 6);
    p.rect(INK, x, y + 5, 14, 17);
    p.rect(STONE_DARK, x + 1, y + 3, 12, 18);
    p.rect(STONE, x + 3, y, 8, 20);
    p.rect(0xa3b29a, x + 5, y - 2, 5, 3);
    p.rect(STONE_LIGHT, x + 3, y + 3, 2, 12);
    p.rect(MOSS, x + 10, y + 6, 3, 5);
    p.rect(0x5f874d, x + 1, y + 16, 4, 4);
    let rune = if obj.collected {
        GOLD
    } else if (time / 0.7).floor() as i32 % 2 == 1 {
        0xa3fff0
    } else {
        CYAN
    };
    p.rect(0x2b999d, x + 5, y + 6, 6, 10);
    p.rect(rune, x + 7, y + 5, 2, 13);
    p.rect(rune, x + 5, y + 8, 6, 2);
    p.rect(rune, x + 4, y + 10, 2, 3);
    p.rect(rune, x + 9, y + 11, 2, 3);
    p.rect(rune, x + 6, y + 14, 4, 2);
    if obj.collected {
        glyph(p, "check", x + 4, y - 9, GOLD);
    } else {
        p.rect(0x366765, x - 2, y + 23, 4, 1);
        p.rect(CYAN, x + 14, y + 20, 1, 2);
    }
}
fn herb(p: &mut Painter, obj: &Object) {
    let (x, y) = (obj.x * TILE + 3, obj.y * TILE + 2);
    p.rect(0x244d3d, x, y + 11, 14, 5);
    p.rect(0x62895b, x + 6, y + 4, 2, 12);
    p.rect(0xa6c780, x + 6, y + 5, 1, 9);
    p.rect(0x31968a, x + 1, y + 6, 5, 3);
    p.rect(0x69c5a2, x + 2, y + 6, 4, 1);
    p.rect(0x31968a, x + 8, y + 10, 5, 3);
    p.rect(0x69c5a2, x + 8, y + 9, 4, 2);
    if !obj.collected {
        p.rect(WHITE, x + 5, y + 1, 4, 3);
        p.rect(GOLD, x + 6, y + 2, 2, 2);
        p.rect(WHITE, x + 11, y + 6, 3, 3);
        p.rect(GOLD, x + 12, y + 7, 1, 1);
    }
}
fn exit(p: &mut Painter, obj: &Object) {
    let (x, y) = (obj.x * TILE, obj.y * TILE);
    p.rect(0x648764, x + 3, y + 2, 15, 16);
    p.rect(0x9ba878, x + 5, y + 5, 11, 10);
    p.rect(INK, x + 5, y + 8, 10, 4);
    p.rect(WHITE, x + 8, y + 9, 7, 2);
    p.rect(WHITE, x + 6, y + 8, 2, 4);
    p.rect(WHITE, x + 4, y + 9, 2, 2);
    p.rect(MOSS, x + 3, y + 15, 4, 3);
}
fn glyph(p: &mut Painter, kind: &str, x: i32, y: i32, color: u32) {
    let map: &[&str] = match kind {
        "strike" | "sword" => &["....1", "...11", "..11.", ".11..", "111..", ".1.1."],
        "bolt" | "arrow" => &[".1...", "..1..", "1111.", "..1..", ".1..."],
        "guard" | "shield" => &["11111", "1...1", "11.11", ".111.", "..1.."],
        "heal" | "herb" => &["..1..", "..1..", "11111", "..1..", "..1.."],
        "check" => &["....1", "...1.", "1.1..", ".1..."],
        "wait" => &[".111.", ".1.1.", ".111.", "..11.", "....."],
        "interact" | "rune" => &["..1..", ".1.1.", "1...1", ".1.1.", "..1.."],
        _ => &["..1..", ".111.", "11111", "..1..", "..1.."],
    };
    for (sy, row) in map.iter().enumerate() {
        for (sx, value) in row.bytes().enumerate() {
            if value == b'1' {
                p.rect(color, x + sx as i32, y + sy as i32, 1, 1);
            }
        }
    }
}
fn corners(p: &mut Painter, x: i32, y: i32, color: u32, inset: i32) {
    let size = TILE - inset * 2;
    let (px, py) = (x * TILE + inset, y * TILE + inset);
    for (dx, dy) in [(0, 0), (size - 5, 0), (0, size - 1), (size - 5, size - 1)] {
        p.rect(color, px + dx, py + dy, 5, 1);
    }
    for (dx, dy) in [(0, 0), (size - 1, 0), (0, size - 5), (size - 1, size - 5)] {
        p.rect(color, px + dx, py + dy, 1, 5);
    }
}
fn actor(p: &mut Painter, actor: &Actor, view: &View) {
    let a = actor;
    let cx = a.x * TILE + 10;
    let feet = a.y * TILE + 18;
    let enemy = a.team == Team::Enemy;
    if a.hp <= 0 {
        p.rect(0x203c38, cx - 6, feet - 3, 13, 4);
        p.rect(
            if enemy { 0x78504c } else { 0x527267 },
            cx - 5,
            feet - 3,
            10,
            2,
        );
        return;
    }
    p.rect(0x1d4239, cx - 8, feet - 1, 16, 3);
    p.rect(
        if enemy { 0xa66250 } else { 0x60958a },
        cx - 6,
        feet + 1,
        12,
        1,
    );
    if a.id == view.selected {
        corners(p, a.x, a.y, GOLD, 1);
    }
    sprite(p, a.role, cx - 9, feet - 22);
    if a.guard > 0 {
        p.rect(0x2c6d78, cx - 9, feet - 12, 3, 8);
        p.rect(0x9de0d7, cx - 9, feet - 13, 3, 1);
        p.rect(0x9de0d7, cx - 9, feet - 7, 1, 3);
    }
    p.rect(INK, cx - 7, feet + 3, 14, 3);
    p.rect(0x344b43, cx - 6, feet + 4, 12, 1);
    let fill = ((12 * a.hp + a.max_hp.max(1) - 1) / a.max_hp.max(1)).clamp(0, 12);
    p.rect(
        if enemy { RED } else { 0xa7d498 },
        cx - 6,
        feet + 4,
        fill,
        1,
    );
    if a.role == Role::Player {
        let y = feet - 30;
        p.rect(INK, cx - 2, y - 1, 5, 7);
        p.rect(GOLD, cx, y, 1, 5);
        p.rect(0xfff1b9, cx - 1, y + 1, 3, 3);
    } else if enemy {
        let kind = match a.intent.kind {
            IntentKind::Attack => "strike",
            IntentKind::Bolt => "bolt",
            IntentKind::Guard => "guard",
            IntentKind::Wait => "wait",
            IntentKind::Move => "move",
        };
        p.rect(INK, cx - 4, feet - 30, 9, 8);
        glyph(p, kind, cx - 2, feet - 29, RED);
    }
}
fn action_kind(action: &Action) -> &'static str {
    match action {
        Action::Move { .. } => "move",
        Action::Strike(_) => "strike",
        Action::Bolt(_) => "bolt",
        Action::Guard => "guard",
        Action::Heal => "heal",
        Action::Wait => "wait",
        Action::Interact(_) => "interact",
    }
}
fn action_tile(state: &State, action: &Action) -> Option<(i32, i32)> {
    match action {
        Action::Move { x, y } => Some((*x, *y)),
        Action::Strike(id) | Action::Bolt(id) => state
            .actors
            .iter()
            .find(|a| a.id == *id)
            .map(|a| (a.x, a.y)),
        _ => state
            .actors
            .iter()
            .find(|a| a.role == Role::Player)
            .map(|a| (a.x, a.y)),
    }
}
fn hover_action(state: &State, view: &View, x: i32, y: i32) -> Action {
    let target = state
        .actors
        .iter()
        .find(|a| a.hp > 0 && a.x == x && a.y == y)
        .map(|a| a.id)
        .unwrap_or(usize::MAX);
    match view.selected_action {
        "strike" => Action::Strike(target),
        "bolt" => Action::Bolt(target),
        "guard" => Action::Guard,
        "heal" => Action::Heal,
        "wait" => Action::Wait,
        "interact" => Action::Interact(None),
        _ => Action::Move { x, y },
    }
}
fn overlays(p: &mut Painter, state: &State, view: &View) {
    if let Some(player) = state
        .actors
        .iter()
        .find(|a| a.role == Role::Player && a.hp > 0)
    {
        let range = match view.selected_action {
            "strike" => 1,
            "bolt" => 5,
            _ => 0,
        };
        for y in 0..state.height {
            for x in 0..state.width {
                let distance = (x - player.x).abs() + (y - player.y).abs();
                if distance > 0 && distance <= range && !blocked(tile(state, x, y)) {
                    p.rect(0x7a9e83, x * TILE + 2, y * TILE + 17, 2, 1);
                    p.rect(0x7a9e83, x * TILE + 17, y * TILE + 2, 1, 2);
                }
            }
        }
    }
    if view.grid {
        p.alpha = 61;
        for x in 0..=state.width {
            p.rect(0xd3d8b2, x * TILE, 0, 1, HEIGHT as i32);
        }
        for y in 0..=state.height {
            p.rect(0xd3d8b2, 0, y * TILE, WIDTH as i32, 1);
        }
        p.alpha = 255;
    }
    if let Some(action) = &view.queued {
        if let Some((x, y)) = action_tile(state, action) {
            p.alpha = 56;
            p.rect(GOLD, x * TILE + 2, y * TILE + 2, 16, 16);
            p.alpha = 255;
            corners(p, x, y, GOLD, 2);
            p.rect(INK, x * TILE + 7, y * TILE + 7, 7, 7);
            glyph(p, action_kind(action), x * TILE + 8, y * TILE + 8, GOLD);
        }
    }
    if let Some((x, y)) = view.hover {
        if x >= 0 && y >= 0 && x < state.width && y < state.height {
            let ok = state.preview(&hover_action(state, view, x, y)).ok;
            let color = if ok { CYAN } else { RED };
            p.alpha = 36;
            p.rect(color, x * TILE + 1, y * TILE + 1, 18, 18);
            p.alpha = 255;
            corners(p, x, y, color, 1);
            if !ok {
                for offset in [8, 10, 12] {
                    p.rect(color, x * TILE + offset, y * TILE + offset, 1, 1);
                }
            }
        }
    }
    let selected = state
        .actors
        .iter()
        .find(|a| a.id == view.selected && a.team == Team::Enemy);
    let hovered = view.hover.and_then(|(x, y)| {
        state
            .actors
            .iter()
            .find(|a| a.team == Team::Enemy && a.x == x && a.y == y)
    });
    if let Some(inspected) = selected.or(hovered) {
        if let Some(target) = inspected
            .intent
            .target_id
            .and_then(|id| state.actors.iter().find(|a| a.id == id && a.hp > 0))
        {
            let (ax, ay) = (inspected.x * TILE + 10, inspected.y * TILE + 10);
            let (bx, by) = (target.x * TILE + 10, target.y * TILE + 10);
            let steps = (bx - ax).abs().max((by - ay).abs());
            if steps > 0 {
                for i in (8..steps - 6).step_by(6) {
                    p.rect(
                        0xc77965,
                        ax + (bx - ax) * i / steps,
                        ay + (by - ay) * i / steps,
                        2,
                        2,
                    );
                }
            }
            corners(p, target.x, target.y, RED, 3);
        }
    }
}
fn digit(value: u8) -> &'static [&'static str] {
    match value {
        b'0' => &["111", "101", "101", "101", "111"],
        b'1' => &["010", "110", "010", "010", "111"],
        b'2' => &["111", "001", "111", "100", "111"],
        b'3' => &["111", "001", "111", "001", "111"],
        b'4' => &["101", "101", "111", "001", "001"],
        b'5' => &["111", "100", "111", "001", "111"],
        b'6' => &["111", "100", "111", "101", "111"],
        b'7' => &["111", "001", "010", "010", "010"],
        b'8' => &["111", "101", "111", "101", "111"],
        b'9' => &["111", "101", "111", "001", "111"],
        b'-' => &["000", "000", "111", "000", "000"],
        b'+' => &["000", "010", "111", "010", "000"],
        _ => &[],
    }
}
fn number(p: &mut Painter, value: &str, x: i32, y: i32, color: u32) {
    for outline in [true, false] {
        for (index, ch) in value.bytes().enumerate() {
            for (sy, row) in digit(ch).iter().enumerate() {
                for (sx, cell) in row.bytes().enumerate() {
                    if cell == b'1' {
                        let (px, py) = (x + index as i32 * 4 + sx as i32, y + sy as i32);
                        if outline {
                            p.rect(INK, px - 1, py - 1, 3, 3);
                        } else {
                            p.rect(color, px, py, 1, 1);
                        }
                    }
                }
            }
        }
    }
}
fn effects(p: &mut Painter, state: &State, view: &View) {
    if view.since_beat > 1.1 {
        return;
    }
    let phase = (view.since_beat / 1.1).clamp(0.0, 1.0);
    for (index, fx) in state.effects.iter().enumerate() {
        let (cx, cy) = (fx.x * TILE + 10, fx.y * TILE + 7);
        let healing = fx.kind == EffectKind::Heal;
        let color = if healing {
            0xb7e6a0
        } else if matches!(fx.kind, EffectKind::Bolt | EffectKind::Interact) {
            CYAN
        } else {
            GOLD
        };
        p.alpha = if phase > 0.65 {
            ((1.0 - phase) / 0.35 * 255.0) as u8
        } else {
            255
        };
        if fx.amount != 0 {
            let value = format!("{}{}", if healing { '+' } else { '-' }, fx.amount.abs());
            number(
                p,
                &value,
                cx - value.len() as i32 * 2 + index as i32 % 2 * 2,
                cy - 17 - (phase * 9.0) as i32,
                if healing { 0xb7e6a0 } else { 0xfff0bb },
            );
        } else if phase < 0.7 {
            let kind = if healing {
                "heal"
            } else if fx.kind == EffectKind::Guard {
                "guard"
            } else {
                "interact"
            };
            glyph(p, kind, cx - 2, cy - 18 - (phase * 6.0) as i32, color);
        }
        if phase < 0.35 {
            let spread = 3 + (phase * 12.0) as i32;
            p.rect(color, cx - spread, cy - 2, 2, 1);
            p.rect(color, cx + spread, cy + 2, 2, 1);
            p.rect(color, cx + 2, cy - spread, 1, 2);
        }
        p.alpha = 255;
    }
}
/// Native 440?240 RGBA8 framebuffer. The host applies nearest sampling and integer zoom.
pub fn render(state: &State, view: &View) -> Vec<u8> {
    let mut p = Painter::new(WIDTH, HEIGHT);
    p.rect(0x153a36, 0, 0, WIDTH as i32, HEIGHT as i32);
    let (width, height) = (state.width.min(22), state.height.min(12));
    for y in 0..height {
        for x in 0..width {
            ground(&mut p, state, x, y, view.time);
        }
    }
    exit(&mut p, &state.exit);
    overlays(&mut p, state, view);
    // Integer depth keys preserve foot-based overlap across scenery and characters.
    let mut objects: Vec<(i32, i32, u8, usize)> = Vec::new();
    for y in 0..height {
        for x in 0..width {
            match tile(state, x, y) {
                '#' => objects.push((y * 10 + 9, x, 0, 0)),
                'R' => objects.push((y * 10 + 8, x, 1, 0)),
                _ => {}
            }
        }
    }
    objects.push((state.objective.y * 10 + 8, state.objective.x, 2, 0));
    objects.push((state.resource.y * 10 + 6, state.resource.x, 3, 0));
    for (i, a) in state.actors.iter().enumerate() {
        objects.push((a.y * 10 + 9, a.x, 4, i));
    }
    objects.sort_by_key(|obj| (obj.0, obj.1, obj.2));
    for (depth, x, kind, index) in objects {
        let y = depth / 10;
        match kind {
            0 => tree(&mut p, x, y),
            1 => {
                if x >= 16 && y <= 4 {
                    masonry(&mut p, x, y, state);
                } else {
                    rubble(&mut p, x, y);
                }
            }
            2 => marker(&mut p, &state.objective, view.time),
            3 => herb(&mut p, &state.resource),
            _ => actor(&mut p, &state.actors[index], view),
        }
    }
    effects(&mut p, state, view);
    p.rect(0x163a36, 0, 0, WIDTH as i32, 1);
    p.rect(0x163a36, 0, HEIGHT as i32 - 1, WIDTH as i32, 1);
    p.rect(0x163a36, 0, 0, 1, HEIGHT as i32);
    p.rect(0x163a36, WIDTH as i32 - 1, 0, 1, HEIGHT as i32);
    p.pixels
}
/// Stable 32?32 RGBA8 role portrait at the same pixel density as the world.
pub fn portrait(role: Role) -> Vec<u8> {
    let mut p = Painter::new(32, 32);
    let enemy = matches!(role, Role::Raider | Role::Archer | Role::Brute);
    p.rect(if enemy { 0x433b3c } else { 0x214c48 }, 0, 0, 32, 32);
    p.rect(if enemy { 0x594447 } else { 0x2b5c52 }, 2, 2, 28, 28);
    p.rect(if enemy { 0x654b49 } else { 0x356955 }, 3, 3, 26, 8);
    p.rect(0x152f34, 3, 27, 26, 2);
    sprite(&mut p, role, 7, 5);
    let color = if enemy { 0xae735c } else { 0x799882 };
    p.rect(color, 1, 1, 30, 1);
    p.rect(color, 1, 1, 1, 30);
    p.rect(0x142e32, 1, 30, 30, 1);
    p.rect(0x142e32, 30, 1, 1, 30);
    p.pixels
}
/// Transparent 16?16 RGBA8 action glyph.
pub fn icon(kind: &str) -> Vec<u8> {
    let mut p = Painter::new(16, 16);
    glyph(
        &mut p,
        kind,
        5,
        5,
        if matches!(kind, "heal" | "herb") {
            0xb7e6a0
        } else {
            GOLD
        },
    );
    p.pixels
}
