use crate::editor::PhoneticEditor;
use crate::parser::Phonetic;

#[test]
fn test_sentences() {
    let p = Phonetic::new();
    assert_eq!("ঘটোৎকচ", p.convert("ghoTOt``kc"));
    assert_eq!("আমার সোনার বাংলা", p.convert("amar sOnar bangla"));
    assert_eq!(
        "আমি বাংলায় গান গাই",
        p.convert("ami banglay gan gai")
    );
    assert_eq!(
        "আমাদের ভালোবাসা হয়ে গেল ঘাস, খেয়ে গেল গরু আর দিয়ে গেল বাঁশ",
        p.convert("amader valObasa hoye gel ghas, kheye gel goru ar diye gelo ba^sh")
    );
}

#[test]
fn basic_test1() {
    let p = Phonetic::new();

    assert_eq!("ভ্ল", p.convert("bhl"));
    assert_eq!("ব্জ", p.convert("bj"));
    assert_eq!("ব্দ", p.convert("bd"));
    assert_eq!("ব্ব", p.convert("bb"));
    assert_eq!("ব্ল", p.convert("bl"));
    assert_eq!("ভ", p.convert("bh"));
    assert_eq!("ভ্ল", p.convert("vl"));
    assert_eq!("ব", p.convert("b"));
    assert_eq!("ভ", p.convert("v"));
    assert_eq!("চ্ঞ", p.convert("cNG"));
    assert_eq!("চ্ছ", p.convert("cch"));
    assert_eq!("চ্চ", p.convert("cc"));
    assert_eq!("ছ", p.convert("ch"));
    assert_eq!("চ", p.convert("c"));
    assert_eq!("ধ্ন", p.convert("dhn"));
    assert_eq!("ধ্ম", p.convert("dhm"));
    assert_eq!("দ্ঘ", p.convert("dgh"));
    assert_eq!("দ্ধ", p.convert("ddh"));
    assert_eq!("দ্ভ", p.convert("dbh"));
    assert_eq!("দ্ভ", p.convert("dv"));
    assert_eq!("দ্ম", p.convert("dm"));
    assert_eq!("ড্ড", p.convert("DD"));
    assert_eq!("ঢ", p.convert("Dh"));
    assert_eq!("ধ", p.convert("dh"));
    assert_eq!("দ্গ", p.convert("dg"));
    assert_eq!("দ্দ", p.convert("dd"));
    assert_eq!("ড", p.convert("D"));
    assert_eq!("দ", p.convert("d"));
}

#[test]
fn basic_test2() {
    let p = Phonetic::new();

    assert_eq!("...", p.convert("..."));
    assert_eq!(".", p.convert(".`"));
    assert_eq!("।।", p.convert(".."));
    assert_eq!("।", p.convert("."));
    assert_eq!("ঘ্ন", p.convert("ghn"));
    assert_eq!("ঘ্ন", p.convert("Ghn"));
    assert_eq!("গ্ধ", p.convert("gdh"));
    assert_eq!("গ্ণ", p.convert("gN"));
    assert_eq!("গ্ণ", p.convert("GN"));
    assert_eq!("গ্ন", p.convert("gn"));
    assert_eq!("গ্ম", p.convert("gm"));
    assert_eq!("গ্ম", p.convert("Gm"));
    assert_eq!("গ্ল", p.convert("gl"));
    assert_eq!("গ্ল", p.convert("Gl"));
    assert_eq!("জ্ঞ", p.convert("gg"));
    assert_eq!("জ্ঞ", p.convert("GG"));
    assert_eq!("জ্ঞ", p.convert("Gg"));
    assert_eq!("জ্ঞ", p.convert("gG"));
    assert_eq!("ঘ", p.convert("gh"));
    assert_eq!("ঘ", p.convert("Gh"));
    assert_eq!("গ", p.convert("g"));
    assert_eq!("হ্ণ", p.convert("hN"));
    assert_eq!("হ্ন", p.convert("hn"));
    assert_eq!("হ্ম", p.convert("hm"));
    assert_eq!("হ্ল", p.convert("hl"));
    assert_eq!("হ", p.convert("h"));
}

#[test]
fn basic_test3() {
    let p = Phonetic::new();

    assert_eq!("জ্ঝ", p.convert("jjh"));
    assert_eq!("জ্ঞ", p.convert("jNG"));
    assert_eq!("ঝ", p.convert("jh"));
    assert_eq!("জ্জ", p.convert("jj"));
    assert_eq!("জ", p.convert("j"));
    assert_eq!("জ", p.convert("J"));
    assert_eq!("ক্ষ্ণ", p.convert("kkhN"));
    assert_eq!("ক্ষ্ণ", p.convert("kShN"));
    assert_eq!("ক্ষ্ম", p.convert("kkhm"));
    assert_eq!("ক্ষ্ম", p.convert("kShm"));
    assert_eq!("ক্ষ্ণ", p.convert("kxN"));
    assert_eq!("ক্ষ্ম", p.convert("kxm"));
    assert_eq!("ক্ষ", p.convert("kkh"));
    assert_eq!("ক্ষ", p.convert("kSh"));
    assert_eq!("কশ", p.convert("ksh"));
    assert_eq!("ক্ষ", p.convert("kx"));
    assert_eq!("ক্ক", p.convert("kk"));
    assert_eq!("ক্ট", p.convert("kT"));
    assert_eq!("ক্ত", p.convert("kt"));
    assert_eq!("ক্ল", p.convert("kl"));
    assert_eq!("ক্স", p.convert("ks"));
    assert_eq!("খ", p.convert("kh"));
    assert_eq!("ক", p.convert("k"));
    assert_eq!("ল্ভ", p.convert("lbh"));
    assert_eq!("ল্ধ", p.convert("ldh"));
    assert_eq!("লখ", p.convert("lkh"));
    assert_eq!("লঘ", p.convert("lgh"));
    assert_eq!("লফ", p.convert("lph"));
    assert_eq!("ল্ক", p.convert("lk"));
    assert_eq!("ল্গ", p.convert("lg"));
    assert_eq!("ল্ট", p.convert("lT"));
    assert_eq!("ল্ড", p.convert("lD"));
    assert_eq!("ল্প", p.convert("lp"));
    assert_eq!("ল্ভ", p.convert("lv"));
    assert_eq!("ল্ম", p.convert("lm"));
    assert_eq!("ল্ল", p.convert("ll"));
    assert_eq!("ল্ব", p.convert("lb"));
    assert_eq!("ল", p.convert("l"));
}

#[test]
fn basic_test4() {
    let p = Phonetic::new();

    assert_eq!("ম্থ", p.convert("mth"));
    assert_eq!("ম্ফ", p.convert("mph"));
    assert_eq!("ম্ভ", p.convert("mbh"));
    assert_eq!("মপ্ল", p.convert("mpl"));
    assert_eq!("ম্ন", p.convert("mn"));
    assert_eq!("ম্প", p.convert("mp"));
    assert_eq!("ম্ভ", p.convert("mv"));
    assert_eq!("ম্ম", p.convert("mm"));
    assert_eq!("ম্ল", p.convert("ml"));
    assert_eq!("ম্ব", p.convert("mb"));
    assert_eq!("ম্ফ", p.convert("mf"));
    assert_eq!("ম", p.convert("m"));
    assert_eq!("০", p.convert("0"));
    assert_eq!("১", p.convert("1"));
    assert_eq!("২", p.convert("2"));
    assert_eq!("৩", p.convert("3"));
    assert_eq!("৪", p.convert("4"));
    assert_eq!("৫", p.convert("5"));
    assert_eq!("৬", p.convert("6"));
    assert_eq!("৭", p.convert("7"));
    assert_eq!("৮", p.convert("8"));
    assert_eq!("৯", p.convert("9"));
    assert_eq!("ঙ্ক্ষ", p.convert("NgkSh"));
    assert_eq!("ঙ্ক্ষ", p.convert("Ngkkh"));
    assert_eq!("ঞ্ছ", p.convert("NGch"));
    assert_eq!("ঙ্ঘ", p.convert("Nggh"));
    assert_eq!("ঙ্খ", p.convert("Ngkh"));
    assert_eq!("ঞ্ঝ", p.convert("NGjh"));
    assert_eq!("ঙ্গৌ", p.convert("ngOU"));
    assert_eq!("ঙ্গৈ", p.convert("ngOI"));
    assert_eq!("ঙ্ক্ষ", p.convert("Ngkx"));
    assert_eq!("ঞ্চ", p.convert("NGc"));
    assert_eq!("ঞ্ছ", p.convert("nch"));
    assert_eq!("ঞ্ঝ", p.convert("njh"));
    assert_eq!("ঙ্ঘ", p.convert("ngh"));
    assert_eq!("ঙ্ক", p.convert("Ngk"));
    assert_eq!("ঙ্ষ", p.convert("Ngx"));
    assert_eq!("ঙ্গ", p.convert("Ngg"));
    assert_eq!("ঙ্ম", p.convert("Ngm"));
    assert_eq!("ঞ্জ", p.convert("NGj"));
    assert_eq!("ন্ধ", p.convert("ndh"));
    assert_eq!("ন্ঠ", p.convert("nTh"));
    assert_eq!("ণ্ঠ", p.convert("NTh"));
    assert_eq!("ন্থ", p.convert("nth"));
    assert_eq!("ঙ্খ", p.convert("nkh"));
    assert_eq!("ঙ্গ", p.convert("ngo"));
    assert_eq!("ঙ্গা", p.convert("nga"));
    assert_eq!("ঙ্গি", p.convert("ngi"));
    assert_eq!("ঙ্গী", p.convert("ngI"));
    assert_eq!("ঙ্গু", p.convert("ngu"));
    assert_eq!("ঙ্গূ", p.convert("ngU"));
    assert_eq!("ঙ্গে", p.convert("nge"));
    assert_eq!("ঙ্গো", p.convert("ngO"));
    assert_eq!("ণ্ঢ", p.convert("NDh"));
    assert_eq!("নশ", p.convert("nsh"));
    assert_eq!("ঙর", p.convert("Ngr"));
    assert_eq!("ঞর", p.convert("NGr"));
    assert_eq!("ংর", p.convert("ngr"));
    assert_eq!("ঞ্জ", p.convert("nj"));
    assert_eq!("ঙ", p.convert("Ng"));
    assert_eq!("ঞ", p.convert("NG"));
    assert_eq!("ঙ্ক", p.convert("nk"));
    assert_eq!("ং", p.convert("ng"));
    assert_eq!("ন্ন", p.convert("nn"));
    assert_eq!("ণ্ণ", p.convert("NN"));
    assert_eq!("ণ্ন", p.convert("Nn"));
    assert_eq!("ন্ম", p.convert("nm"));
    assert_eq!("ণ্ম", p.convert("Nm"));
    assert_eq!("ন্দ", p.convert("nd"));
    assert_eq!("ন্ট", p.convert("nT"));
    assert_eq!("ণ্ট", p.convert("NT"));
    assert_eq!("ন্ড", p.convert("nD"));
    assert_eq!("ণ্ড", p.convert("ND"));
    assert_eq!("ন্ত", p.convert("nt"));
    assert_eq!("ন্স", p.convert("ns"));
    assert_eq!("ঞ্চ", p.convert("nc"));
    assert_eq!("ন", p.convert("n"));
    assert_eq!("ণ", p.convert("N"));
}

#[test]
fn basic_test5() {
    let p = Phonetic::new();

    assert_eq!("ৈ", p.convert("OI`"));
    assert_eq!("ৌ", p.convert("OU`"));
    assert_eq!("ো", p.convert("O`"));
    assert_eq!("ঐ", p.convert("OI"));
    assert_eq!("কৈ", p.convert("kOI"));
    assert_eq!(" ঐ", p.convert(" OI"));
    assert_eq!("(ঐ", p.convert("(OI"));
    assert_eq!("।ঐ", p.convert(".OI"));
    assert_eq!("ঔ", p.convert("OU"));
    assert_eq!("কৌ", p.convert("kOU"));
    assert_eq!(" ঔ", p.convert(" OU"));
    assert_eq!("-ঔ", p.convert("-OU"));
    assert_eq!("্‌ঔ", p.convert(",,OU"));
    assert_eq!("ও", p.convert("O"));
    assert_eq!("পো", p.convert("pO"));
    assert_eq!(" ও", p.convert(" O"));
    assert_eq!("ইও", p.convert("iO"));
    assert_eq!("ও", p.convert("`O"));
    assert_eq!("ফ্ল", p.convert("phl"));
    assert_eq!("প্ট", p.convert("pT"));
    assert_eq!("প্ত", p.convert("pt"));
    assert_eq!("প্ন", p.convert("pn"));
    assert_eq!("প্প", p.convert("pp"));
    assert_eq!("প্ল", p.convert("pl"));
    assert_eq!("প্স", p.convert("ps"));
    assert_eq!("ফ", p.convert("ph"));
    assert_eq!("ফ্ল", p.convert("fl"));
    assert_eq!("ফ", p.convert("f"));
    assert_eq!("প", p.convert("p"));
    assert_eq!("ৃ", p.convert("rri`"));
    assert_eq!("ঋ", p.convert("rri"));
    assert_eq!("কৃ", p.convert("krri"));
    assert_eq!("ঈঋ", p.convert("Irri"));
    assert_eq!("ঁঋ", p.convert("^rri"));
    assert_eq!("ঃঋ", p.convert(":rri"));
    assert_eq!("র‍্য", p.convert("rZ"));
}

#[test]
fn basic_test6() {
    let p = Phonetic::new();

    assert_eq!("ক্র্য", p.convert("krZ"));
    assert_eq!("রর‍্য", p.convert("rrZ"));
    assert_eq!("ইয়র‍্য", p.convert("yrZ"));
    assert_eq!("ওর‍্য", p.convert("wrZ"));
    assert_eq!("এক্সর‍্য", p.convert("xrZ"));
    assert_eq!("ইর‍্য", p.convert("irZ"));
    assert_eq!("-র‍্য", p.convert("-rZ"));
    assert_eq!("ররর‍্য", p.convert("rrrZ"));
    assert_eq!("র‍্য", p.convert("ry"));
    assert_eq!("ক্র্য", p.convert("qry"));
    assert_eq!("রর‍্য", p.convert("rry"));
    assert_eq!("ইয়র‍্য", p.convert("yry"));
    assert_eq!("ওর‍্য", p.convert("wry"));
    assert_eq!("এক্সর‍্য", p.convert("xry"));
    assert_eq!("০র‍্য", p.convert("0ry"));
    assert_eq!("রররর‍্য", p.convert("rrrry"));
    assert_eq!("ড়্র্য", p.convert("Rry"));
    assert_eq!("রর", p.convert("rr"));
    assert_eq!("আরর", p.convert("arr"));
    assert_eq!("আর্ক", p.convert("arrk"));
    assert_eq!("আররা", p.convert("arra"));
    assert_eq!("আরর", p.convert("arr"));
    assert_eq!("আরর!", p.convert("arr!"));
    assert_eq!("ক্রর", p.convert("krr"));
    assert_eq!("ক্ররা", p.convert("krra"));
    assert_eq!("ড়্গ", p.convert("Rg"));
    assert_eq!("ঢ়", p.convert("Rh"));
    assert_eq!("ড়", p.convert("R"));
    assert_eq!("র", p.convert("r"));
    assert_eq!("অর", p.convert("or"));
    assert_eq!("ম্র", p.convert("mr"));
    assert_eq!("১র", p.convert("1r"));
    assert_eq!("+র", p.convert("+r"));
    assert_eq!("রর", p.convert("rr"));
    assert_eq!("ইয়র", p.convert("yr"));
    assert_eq!("ওর", p.convert("wr"));
    assert_eq!("এক্সর", p.convert("xr"));
    assert_eq!("য্র", p.convert("zr"));
    assert_eq!("ম্রি", p.convert("mri"));
}

#[test]
fn basic_test7() {
    let p = Phonetic::new();

    assert_eq!("শ্ছ", p.convert("shch"));
    assert_eq!("ষ্ঠ", p.convert("ShTh"));
    assert_eq!("ষ্ফ", p.convert("Shph"));
    assert_eq!("শ্ছ", p.convert("Sch"));
    assert_eq!("স্ক্ল", p.convert("skl"));
    assert_eq!("স্খ", p.convert("skh"));
    assert_eq!("স্থ", p.convert("sth"));
    assert_eq!("স্ফ", p.convert("sph"));
    assert_eq!("শ্চ", p.convert("shc"));
    assert_eq!("শ্ত", p.convert("sht"));
    assert_eq!("শ্ন", p.convert("shn"));
    assert_eq!("শ্ম", p.convert("shm"));
    assert_eq!("শ্ল", p.convert("shl"));
    assert_eq!("ষ্ক", p.convert("Shk"));
    assert_eq!("ষ্ট", p.convert("ShT"));
    assert_eq!("ষ্ণ", p.convert("ShN"));
    assert_eq!("ষ্প", p.convert("Shp"));
    assert_eq!("ষ্ফ", p.convert("Shf"));
    assert_eq!("ষ্ম", p.convert("Shm"));
    assert_eq!("স্প্ল", p.convert("spl"));
    assert_eq!("স্ক", p.convert("sk"));
    assert_eq!("শ্চ", p.convert("Sc"));
    assert_eq!("স্ট", p.convert("sT"));
    assert_eq!("স্ত", p.convert("st"));
    assert_eq!("স্ন", p.convert("sn"));
    assert_eq!("স্প", p.convert("sp"));
    assert_eq!("স্ফ", p.convert("sf"));
    assert_eq!("স্ম", p.convert("sm"));
    assert_eq!("স্ল", p.convert("sl"));
    assert_eq!("শ", p.convert("sh"));
    assert_eq!("শ্চ", p.convert("Sc"));
    assert_eq!("শ্ত", p.convert("St"));
    assert_eq!("শ্ন", p.convert("Sn"));
    assert_eq!("শ্ম", p.convert("Sm"));
    assert_eq!("শ্ল", p.convert("Sl"));
    assert_eq!("ষ", p.convert("Sh"));
    assert_eq!("স", p.convert("s"));
    assert_eq!("শ", p.convert("S"));
    assert_eq!("উ", p.convert("oo"));
    assert_eq!("ওও", p.convert("OO"));
    assert_eq!("ু", p.convert("oo`"));
    assert_eq!("কু", p.convert("koo"));
    assert_eq!("উঅ", p.convert("ooo"));
    assert_eq!("!উ", p.convert("!oo"));
    assert_eq!("!উঅ", p.convert("!ooo"));
    assert_eq!("আউ", p.convert("aoo"));
    assert_eq!("উপ", p.convert("oop"));
    assert_eq!("উ", p.convert("ooo`"));
    assert_eq!("", p.convert("o`"));
    assert_eq!("অ্য", p.convert("oZ"));
    assert_eq!("অয়", p.convert("oY"));
    assert_eq!("অ", p.convert("o"));
    assert_eq!("!অ", p.convert("!o"));
    assert_eq!("ঁঅ", p.convert("^o"));
    assert_eq!("*অ", p.convert("*o"));
    assert_eq!("ইও", p.convert("io"));
    assert_eq!("ইয়", p.convert("yo"));
    assert_eq!("ন", p.convert("no"));
    assert_eq!("ত্থ", p.convert("tth"));
    assert_eq!("ৎ", p.convert("t``"));
    assert_eq!("ৎ", p.convert("`t``"));
    assert_eq!("ৎৎ", p.convert("t``t``"));
    assert_eq!("ৎ", p.convert("t```"));
    assert_eq!("ট্ট", p.convert("TT"));
    assert_eq!("ট্ম", p.convert("Tm"));
    assert_eq!("ঠ", p.convert("Th"));
    assert_eq!("ত্ন", p.convert("tn"));
    assert_eq!("ত্ম", p.convert("tm"));
    assert_eq!("থ", p.convert("th"));
    assert_eq!("ত্ত", p.convert("tt"));
    assert_eq!("ট", p.convert("T"));
    assert_eq!("ত", p.convert("t"));
}

#[test]
fn basic_test8() {
    let p = Phonetic::new();

    assert_eq!("অ্যা", p.convert("aZ"));
    assert_eq!("আঅ্যা", p.convert("aaZ"));
    assert_eq!("অ্যা", p.convert("AZ"));
    assert_eq!("া", p.convert("a`"));
    assert_eq!("া", p.convert("a``"));
    assert_eq!("কা", p.convert("ka`"));
    assert_eq!("া", p.convert("A`"));
    assert_eq!("আ", p.convert("a"));
    assert_eq!("আ", p.convert("`a"));
    assert_eq!("কআ", p.convert("k`a"));
    assert_eq!("ইয়া", p.convert("ia"));
    assert_eq!("আআআা", p.convert("aaaa`"));
    assert_eq!("ি", p.convert("i`"));
    assert_eq!("ই", p.convert("i"));
    assert_eq!("ই", p.convert("`i"));
    assert_eq!("হি", p.convert("hi"));
    assert_eq!("ইহ", p.convert("ih"));
    assert_eq!("িহ", p.convert("i`h"));
    assert_eq!("ী", p.convert("I`"));
    assert_eq!("ঈ", p.convert("I"));
    assert_eq!("চী", p.convert("cI"));
    assert_eq!("ঈক্স", p.convert("Ix"));
    assert_eq!("ঈঈ", p.convert("II"));
    assert_eq!("০ঈ", p.convert("0I"));
    assert_eq!("অঈ", p.convert("oI"));
    assert_eq!("ু", p.convert("u`"));
    assert_eq!("উ", p.convert("u"));
    assert_eq!("কু", p.convert("ku"));
    assert_eq!("উক", p.convert("uk"));
    assert_eq!("উউ", p.convert("uu"));
    assert_eq!("ইউ", p.convert("iu"));
    assert_eq!("&উ", p.convert("&u"));
    assert_eq!("উ&", p.convert("u&"));
    assert_eq!("ূ", p.convert("U`"));
    assert_eq!("ঊ", p.convert("U"));
    assert_eq!("ইয়ূ", p.convert("yU"));
    assert_eq!("ঊয়", p.convert("Uy"));
    assert_eq!("ঁঊ", p.convert("^U"));
    assert_eq!("ঊঁ", p.convert("U^"));
    assert_eq!("ঈ", p.convert("EE"));
    assert_eq!("ঈ", p.convert("ee"));
    assert_eq!("ঈ", p.convert("Ee"));
    assert_eq!("ঈ", p.convert("eE"));
    assert_eq!("ী", p.convert("ee`"));
    assert_eq!("কী", p.convert("kee"));
    assert_eq!("ঈক", p.convert("eek"));
    assert_eq!("০ঈ", p.convert("0ee"));
    assert_eq!("ঈ৮", p.convert("ee8"));
    assert_eq!("(ঈ)", p.convert("(ee)"));
    assert_eq!("ে", p.convert("e`"));
    assert_eq!("এ", p.convert("e"));
    assert_eq!("কে", p.convert("ke"));
    assert_eq!("ওয়ে", p.convert("we"));
    assert_eq!("#এ#", p.convert("#e#"));
    assert_eq!("ে", p.convert("`e`"));
    assert_eq!("য", p.convert("z"));
    assert_eq!("্য", p.convert("Z"));
    assert_eq!("র‍্য", p.convert("rZ"));
    assert_eq!("ক্যশ", p.convert("kZS"));
    assert_eq!("ইয়", p.convert("y"));
    assert_eq!("অয়", p.convert("oy"));
    assert_eq!("ক্য", p.convert("ky"));
    assert_eq!("ইয়া", p.convert("ya"));
    assert_eq!("ইয়াআ", p.convert("yaa"));
    assert_eq!("য়", p.convert("Y"));
    assert_eq!("য়য়", p.convert("YY"));
    assert_eq!("ইয়", p.convert("iY"));
    assert_eq!("কয়", p.convert("kY"));
    assert_eq!("ক", p.convert("q"));
    assert_eq!("ক", p.convert("Q"));
    assert_eq!("ও", p.convert("w"));
    assert_eq!("ওয়া", p.convert("wa"));
    assert_eq!("-ওয়া-", p.convert("-wa-"));
    assert_eq!("ওয়ু", p.convert("woo"));
    assert_eq!("ওরে", p.convert("wre"));
    assert_eq!("ক্ব", p.convert("kw"));
    assert_eq!("এক্স", p.convert("x"));
    assert_eq!("এক্স", p.convert("ex"));
    assert_eq!("বক্স", p.convert("bx"));
    assert_eq!(":", p.convert(":`"));
    assert_eq!("ঃ", p.convert(":"));
    assert_eq!("^", p.convert("^`"));
    assert_eq!("ঁ", p.convert("^"));
    assert_eq!("কঁ", p.convert("k^"));
    assert_eq!("কঁই", p.convert("k^i"));
    assert_eq!("কিঁ", p.convert("ki^"));
    assert_eq!("্‌", p.convert(",,"));
    assert_eq!("্‌,", p.convert(",,,"));
    assert_eq!("্‌,", p.convert(",,`,"));
    assert_eq!("্‌", p.convert("`,,"));
    assert_eq!(",,", p.convert(",`,"));
    assert_eq!("৳", p.convert("$"));
    assert_eq!("", p.convert("`"));
    assert_eq!("ব্ধ", p.convert("bdh"));
}

#[test]
fn editor_test() {
    let mut editor = PhoneticEditor::new();

    let result = editor.put_new_ch('k', 1);
    assert_eq!("ক", result.output);

    let result = editor.put_new_ch('o', 2);
    assert_eq!("ক", result.output);

    let result = editor.put_new_ch('r', 2);
    assert_eq!("কর", result.output);

    let result = editor.put_new_ch('r', 3);
    assert_eq!("করর", result.output);

    let result = editor.put_new_ch('m', 4);
    assert_eq!("কর্ম", result.output);

    let result = editor.put_new_ch(' ', 0);
    assert_eq!("", result.output);
}
