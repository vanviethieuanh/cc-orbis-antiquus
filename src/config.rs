use bevy::{ecs::resource::Resource, math::Vec2, text::Font};

#[derive(Debug, Resource)]
pub struct CanvasConfig {
    pub size: Vec2,
    pub border_thickness: f32,
    pub margin: Edges,
}
impl CanvasConfig {
    pub fn left(&self) -> f32 {
        -self.size.x / 2.0 + self.border_thickness
    }

    pub fn right(&self) -> f32 {
        self.size.x / 2.0 - self.border_thickness
    }

    pub fn top(&self) -> f32 {
        self.size.y / 2.0 - self.border_thickness
    }

    pub fn bottom(&self) -> f32 {
        -self.size.y / 2.0 + self.border_thickness
    }
}

#[derive(Debug, Resource)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Resource)]
pub struct ZIndexConfig {
    pub paper: f32,
    pub map: f32,
    pub overlays: f32,
}

#[derive(Debug, Resource)]
pub struct PolarConfig {
    pub radius: f32,

    pub ring_thickness: f32,
    pub ring_spacing_factor: f32,

    pub stroke_thickness: f32,
    pub lim_lat: f32,
}

impl PolarConfig {
    pub fn offset(&self) -> f32 {
        self.ring_thickness * self.ring_spacing_factor
    }
}

#[derive(Debug, Resource)]
pub struct MainMapConfig {
    pub median_segments: usize,
    pub borders_spacings: Vec<f32>,
}

#[derive(Debug, Resource)]
pub struct DataConfig {
    pub shape_filepath: &'static str,
}

#[derive(Debug, Resource)]
pub struct NoteSpacingConfig {
    pub main: f32,
}

#[derive(Debug, Resource)]
pub struct NoteConfig {
    pub font_size: f32,
    pub text: &'static str,
}

#[derive(Debug, Resource)]
pub struct FontConfig {
    pub bold: &'static str,
    pub regular: &'static str,
    pub light: &'static str,
}

#[derive(Debug, Resource)]
pub struct NotesConfig {
    pub font: FontConfig,
    pub spacing: NoteSpacingConfig,
    pub title: NoteConfig,
    pub under_title: NoteConfig,
}

#[derive(Debug, Resource)]
pub struct MapConfig {
    pub canvas: CanvasConfig,
    pub z: ZIndexConfig,
    pub polar: PolarConfig,
    pub main_map: MainMapConfig,
    pub data: DataConfig,
    pub note: NotesConfig,
}

impl Default for MapConfig {
    fn default() -> Self {
        static UNDER_TITLE_TEXT: &'static str = "地與海本是圓形而合為一球居天球之中誠如雞子黃在青內有謂地為方者乃語其定而不移之性非語其形體也天既包地則彼此相應故天有南北二極地亦有之天分三百六十度地亦同之天中有赤道自赤道而南二十三度半為南道赤道而北二十三度半為北道按中國在赤道之北日行赤道則晝夜平行南道則晝短行北道則晝長故天球有晝夜平圈列於中晝短晝長二圈列於南北以著日行之界地球亦設三圈對於下焉但天包地外為甚大其度廣地處天中為甚小其度狹此其差異者耳查得直行北方者每路二百五十里覺北極出高一度南極入低一度直行南方者每路二百五十里覺北極入低一度南極出高一度則不特審地形果圓而並徵地之每一度廣二百五十里則地之東西南北各一周有九萬里實數也是南北與東西數相等而不容異也夫地厚二萬八千六百三十六里零百分里之三十六分上下四旁皆生齒所居渾淪一球原無上下蓋在天之內何瞻非天!總六合內凡足所佇即為下凡首所向即為上其專以身之所居分上下者未然也且予自大西浮海入中國至晝夜平線已見南北二極皆在平地略無高低道轉而南過大浪山已見南極出地三十五度則大浪山與中國上下相為對待矣而吾彼時只仰天在上未視之在下也故謂地形圓而周圍皆生齒者信然矣以天勢分山海自北而南為五帶一在晝長晝短二圈之間其地甚熱帶近日輪故也二在北極圈之內三在南極圈之內此二處地居甚冷帶遠日輪故也四在北極晝長二圈之間五在南極晝短二圈之間此二地皆謂之正帶不甚冷熱日輪不遠不近故也又以地勢分輿地為五大州曰歐邏巴曰利未亞曰亞細亞曰南北亞墨利加曰墨瓦蠟泥加若歐邏巴者南至地中海北至青地及冰海東至大乃河墨阿的湖大海西至大西洋若利未亞者南至大浪山北至地中海東至西紅海聖老楞佐島西至阿則亞諾海即此州只以聖土之下微路與亞細亞相聯其餘全為四海所圍若亞細亞者南至蘇門答喇呂宋等島北至新增白臘及北海東至日本島大清海西至大乃河墨阿的湖大海西紅海小西洋若亞墨利加者全為四海所圍南北以微地相聯若瑪熱辣泥加者盡在南方惟見南極出地而北極恒藏焉其界未審何如故未敢訂之惟其北邊與爪哇及瑪熱辣泥峽為境也其各州之界當以五色別之令其便覽各國繁夥難悉原宜作圓球以其入圖不便不得不易圓為平反圈為線耳欲知其形必須相合連東西二海為一片可也其經緯線本宜每度畫之今且惟每十度為一方以免雜亂依是可分置各國于其所天下之緯自晝夜平線為中而起上數至北極下數至南極天下之經自順天府起為初度至三百六十度復相接焉試如察得福島離中線以上二十八度離順天府以東二百十五度則安之于所也凡地在中線以上至北極則實為北方凡在中線以下則實為南方焉又用緯線以著各極出地幾何蓋地離晝夜平線度數與極出地度數相等但在南方則著南極出地之數在北方則著北極出地之數也假如視京師隔中線以北四十度則知京師北極高四十度也視大浪山隔中線以南三十五度則知大浪山南極高三十五度也凡同緯之地其極出地數同則四季寒暑同態焉若兩處離中線度數相同但一離于南一離于北其四季並晝夜刻數均同惟時相反此之夏為彼之冬耳其長晝長夜離中線愈遠則其長愈多餘為式以記于圖邊每五度其晝夜長何如則東西上下隔中線數一則皆可通用焉用經線以定兩處相離幾何辰也蓋日輪一日作一周則每辰行三十度兩處相離三十度並謂差一辰假如山西太原府列在于三百五十五經度而則意蘭島列于三百二十五經度彼此相去三十度則相差一辰故凡太原為午則意蘭為巳其餘仿此焉設差六辰則兩處晝夜相反焉如所離中線度數又同而差南北則兩地人對足底反行假如河南開封府離中線以北三十四度而列在于三百五十七經度又南亞墨利加之內近銀河之地如趙路亞斯等離中線以南三十四度而列于一百七十七經度彼此相去一百八十度即六辰則彼此相對反足底行矣從此可曉同經線處並同辰而同時見日月蝕焉夫地圖所定各方之經緯度多歷年世愈久而愈準蓋其定法以測驗為主當其始天下大半諸國地及海島不可更僕前無紀錄之書不知海外之復有此大地否也近今二百年來大西洋諸國名士航海通遊天下周圍無所不到凡各地依歷學諸法測天以定本地經緯度是以萬國地名輿圖大備如此其六合之地及山川江河湖海島嶼原無名稱凡初歷其地者多以前古聖人之名名之以為別識而定其道里云";
        Self {
            canvas: CanvasConfig {
                size: Vec2::new(11100., 5000.),
                border_thickness: 20.,
                margin: Edges {
                    top: 40.,
                    right: 40.,
                    bottom: 40.,
                    left: 40.,
                },
            },
            z: ZIndexConfig {
                paper: 1.0,
                map: 2.0,
                overlays: 3.0,
            },
            polar: PolarConfig {
                radius: 560.0,

                ring_thickness: 6.0,
                ring_spacing_factor: 3.0,
                stroke_thickness: 0.5,

                lim_lat: 30.0,
            },
            main_map: MainMapConfig {
                median_segments: 64,
                borders_spacings: vec![12.0, 36.0, 60.0],
            },
            data: DataConfig {
                shape_filepath: "data/raw/natural_earth/ne_110m_land/ne_110m_land.shp",
            },
            note: NotesConfig {
                spacing: NoteSpacingConfig { main: 10.0 },
                font: FontConfig {
                    regular: "fonts/LXGWWenKaiTC-Regular.ttf",
                    bold: "fonts/LXGWWenKaiTC-Bold.ttf",
                    light: "fonts/LXGWWenKaiTC-Light.ttf",
                },
                title: NoteConfig {
                    font_size: 160.0,
                    text: "坤輿萬國全圖",
                },
                under_title: NoteConfig {
                    font_size: 28.0,
                    text: UNDER_TITLE_TEXT,
                },
            },
        }
    }
}
