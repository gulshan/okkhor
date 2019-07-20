use crate::editor::PhoneticEditor;
use crate::parser::parse;

#[test]
fn test_sentences() {
    assert_eq!("ঘটোৎকচ", parse("ghoTOt``kc"));
    assert_eq!(
        "আমি বাংলায় গান গাই",
        parse("ami banglay gan gai")
    );
    assert_eq!(
        "আমাদের ভালোবাসা হয়ে গেল ঘাস, খেয়ে গেল গরু আর দিয়ে গেল বাঁশ",
        parse("amader valObasa hoye gel ghas, kheye gel goru ar diye gelo ba^sh")
    );
}

#[test]
fn basic_test1() {
    assert_eq!("ভ্ল", parse("bhl"));
    assert_eq!("ব্জ", parse("bj"));
    assert_eq!("ব্দ", parse("bd"));
    assert_eq!("ব্ব", parse("bb"));
    assert_eq!("ব্ল", parse("bl"));
    assert_eq!("ভ", parse("bh"));
    assert_eq!("ভ্ল", parse("vl"));
    assert_eq!("ব", parse("b"));
    assert_eq!("ভ", parse("v"));
    assert_eq!("চ্ঞ", parse("cNG"));
    assert_eq!("চ্ছ", parse("cch"));
    assert_eq!("চ্চ", parse("cc"));
    assert_eq!("ছ", parse("ch"));
    assert_eq!("চ", parse("c"));
    assert_eq!("ধ্ন", parse("dhn"));
    assert_eq!("ধ্ম", parse("dhm"));
    assert_eq!("দ্ঘ", parse("dgh"));
    assert_eq!("দ্ধ", parse("ddh"));
    assert_eq!("দ্ভ", parse("dbh"));
    assert_eq!("দ্ভ", parse("dv"));
    assert_eq!("দ্ম", parse("dm"));
    assert_eq!("ড্ড", parse("DD"));
    assert_eq!("ঢ", parse("Dh"));
    assert_eq!("ধ", parse("dh"));
    assert_eq!("দ্গ", parse("dg"));
    assert_eq!("দ্দ", parse("dd"));
    assert_eq!("ড", parse("D"));
    assert_eq!("দ", parse("d"));
}

#[test]
fn basic_test2() {
    assert_eq!("...", parse("..."));
    assert_eq!(".", parse(".`"));
    assert_eq!("।।", parse(".."));
    assert_eq!("।", parse("."));
    assert_eq!("ঘ্ন", parse("ghn"));
    assert_eq!("ঘ্ন", parse("Ghn"));
    assert_eq!("গ্ধ", parse("gdh"));
    assert_eq!("গ্ণ", parse("gN"));
    assert_eq!("গ্ণ", parse("GN"));
    assert_eq!("গ্ন", parse("gn"));
    assert_eq!("গ্ম", parse("gm"));
    assert_eq!("গ্ম", parse("Gm"));
    assert_eq!("গ্ল", parse("gl"));
    assert_eq!("গ্ল", parse("Gl"));
    assert_eq!("জ্ঞ", parse("gg"));
    assert_eq!("জ্ঞ", parse("GG"));
    assert_eq!("জ্ঞ", parse("Gg"));
    assert_eq!("জ্ঞ", parse("gG"));
    assert_eq!("ঘ", parse("gh"));
    assert_eq!("ঘ", parse("Gh"));
    assert_eq!("গ", parse("g"));
    assert_eq!("হ্ণ", parse("hN"));
    assert_eq!("হ্ন", parse("hn"));
    assert_eq!("হ্ম", parse("hm"));
    assert_eq!("হ্ল", parse("hl"));
    assert_eq!("হ", parse("h"));
}

#[test]
fn basic_test3() {
    assert_eq!("জ্ঝ", parse("jjh"));
    assert_eq!("জ্ঞ", parse("jNG"));
    assert_eq!("ঝ", parse("jh"));
    assert_eq!("জ্জ", parse("jj"));
    assert_eq!("জ", parse("j"));
    assert_eq!("জ", parse("J"));
    assert_eq!("ক্ষ্ণ", parse("kkhN"));
    assert_eq!("ক্ষ্ণ", parse("kShN"));
    assert_eq!("ক্ষ্ম", parse("kkhm"));
    assert_eq!("ক্ষ্ম", parse("kShm"));
    assert_eq!("ক্ষ্ণ", parse("kxN"));
    assert_eq!("ক্ষ্ম", parse("kxm"));
    assert_eq!("ক্ষ", parse("kkh"));
    assert_eq!("ক্ষ", parse("kSh"));
    assert_eq!("কশ", parse("ksh"));
    assert_eq!("ক্ষ", parse("kx"));
    assert_eq!("ক্ক", parse("kk"));
    assert_eq!("ক্ট", parse("kT"));
    assert_eq!("ক্ত", parse("kt"));
    assert_eq!("ক্ল", parse("kl"));
    assert_eq!("ক্স", parse("ks"));
    assert_eq!("খ", parse("kh"));
    assert_eq!("ক", parse("k"));
    assert_eq!("ল্ভ", parse("lbh"));
    assert_eq!("ল্ধ", parse("ldh"));
    assert_eq!("লখ", parse("lkh"));
    assert_eq!("লঘ", parse("lgh"));
    assert_eq!("লফ", parse("lph"));
    assert_eq!("ল্ক", parse("lk"));
    assert_eq!("ল্গ", parse("lg"));
    assert_eq!("ল্ট", parse("lT"));
    assert_eq!("ল্ড", parse("lD"));
    assert_eq!("ল্প", parse("lp"));
    assert_eq!("ল্ভ", parse("lv"));
    assert_eq!("ল্ম", parse("lm"));
    assert_eq!("ল্ল", parse("ll"));
    assert_eq!("ল্ব", parse("lb"));
    assert_eq!("ল", parse("l"));
}

#[test]
fn basic_test4() {
    assert_eq!("ম্থ", parse("mth"));
    assert_eq!("ম্ফ", parse("mph"));
    assert_eq!("ম্ভ", parse("mbh"));
    assert_eq!("মপ্ল", parse("mpl"));
    assert_eq!("ম্ন", parse("mn"));
    assert_eq!("ম্প", parse("mp"));
    assert_eq!("ম্ভ", parse("mv"));
    assert_eq!("ম্ম", parse("mm"));
    assert_eq!("ম্ল", parse("ml"));
    assert_eq!("ম্ব", parse("mb"));
    assert_eq!("ম্ফ", parse("mf"));
    assert_eq!("ম", parse("m"));
    assert_eq!("০", parse("0"));
    assert_eq!("১", parse("1"));
    assert_eq!("২", parse("2"));
    assert_eq!("৩", parse("3"));
    assert_eq!("৪", parse("4"));
    assert_eq!("৫", parse("5"));
    assert_eq!("৬", parse("6"));
    assert_eq!("৭", parse("7"));
    assert_eq!("৮", parse("8"));
    assert_eq!("৯", parse("9"));
    assert_eq!("ঙ্ক্ষ", parse("NgkSh"));
    assert_eq!("ঙ্ক্ষ", parse("Ngkkh"));
    assert_eq!("ঞ্ছ", parse("NGch"));
    assert_eq!("ঙ্ঘ", parse("Nggh"));
    assert_eq!("ঙ্খ", parse("Ngkh"));
    assert_eq!("ঞ্ঝ", parse("NGjh"));
    assert_eq!("ঙ্গৌ", parse("ngOU"));
    assert_eq!("ঙ্গৈ", parse("ngOI"));
    assert_eq!("ঙ্ক্ষ", parse("Ngkx"));
    assert_eq!("ঞ্চ", parse("NGc"));
    assert_eq!("ঞ্ছ", parse("nch"));
    assert_eq!("ঞ্ঝ", parse("njh"));
    assert_eq!("ঙ্ঘ", parse("ngh"));
    assert_eq!("ঙ্ক", parse("Ngk"));
    assert_eq!("ঙ্ষ", parse("Ngx"));
    assert_eq!("ঙ্গ", parse("Ngg"));
    assert_eq!("ঙ্ম", parse("Ngm"));
    assert_eq!("ঞ্জ", parse("NGj"));
    assert_eq!("ন্ধ", parse("ndh"));
    assert_eq!("ন্ঠ", parse("nTh"));
    assert_eq!("ণ্ঠ", parse("NTh"));
    assert_eq!("ন্থ", parse("nth"));
    assert_eq!("ঙ্খ", parse("nkh"));
    assert_eq!("ঙ্গ", parse("ngo"));
    assert_eq!("ঙ্গা", parse("nga"));
    assert_eq!("ঙ্গি", parse("ngi"));
    assert_eq!("ঙ্গী", parse("ngI"));
    assert_eq!("ঙ্গু", parse("ngu"));
    assert_eq!("ঙ্গূ", parse("ngU"));
    assert_eq!("ঙ্গে", parse("nge"));
    assert_eq!("ঙ্গো", parse("ngO"));
    assert_eq!("ণ্ঢ", parse("NDh"));
    assert_eq!("নশ", parse("nsh"));
    assert_eq!("ঙর", parse("Ngr"));
    assert_eq!("ঞর", parse("NGr"));
    assert_eq!("ংর", parse("ngr"));
    assert_eq!("ঞ্জ", parse("nj"));
    assert_eq!("ঙ", parse("Ng"));
    assert_eq!("ঞ", parse("NG"));
    assert_eq!("ঙ্ক", parse("nk"));
    assert_eq!("ং", parse("ng"));
    assert_eq!("ন্ন", parse("nn"));
    assert_eq!("ণ্ণ", parse("NN"));
    assert_eq!("ণ্ন", parse("Nn"));
    assert_eq!("ন্ম", parse("nm"));
    assert_eq!("ণ্ম", parse("Nm"));
    assert_eq!("ন্দ", parse("nd"));
    assert_eq!("ন্ট", parse("nT"));
    assert_eq!("ণ্ট", parse("NT"));
    assert_eq!("ন্ড", parse("nD"));
    assert_eq!("ণ্ড", parse("ND"));
    assert_eq!("ন্ত", parse("nt"));
    assert_eq!("ন্স", parse("ns"));
    assert_eq!("ঞ্চ", parse("nc"));
    assert_eq!("ন", parse("n"));
    assert_eq!("ণ", parse("N"));
}

#[test]
fn basic_test5() {
    assert_eq!("ৈ", parse("OI`"));
    assert_eq!("ৌ", parse("OU`"));
    assert_eq!("ো", parse("O`"));
    assert_eq!("ঐ", parse("OI"));
    assert_eq!("কৈ", parse("kOI"));
    assert_eq!(" ঐ", parse(" OI"));
    assert_eq!("(ঐ", parse("(OI"));
    assert_eq!("।ঐ", parse(".OI"));
    assert_eq!("ঔ", parse("OU"));
    assert_eq!("কৌ", parse("kOU"));
    assert_eq!(" ঔ", parse(" OU"));
    assert_eq!("-ঔ", parse("-OU"));
    assert_eq!("্‌ঔ", parse(",,OU"));
    assert_eq!("ও", parse("O"));
    assert_eq!("পো", parse("pO"));
    assert_eq!(" ও", parse(" O"));
    assert_eq!("ইও", parse("iO"));
    assert_eq!("ও", parse("`O"));
    assert_eq!("ফ্ল", parse("phl"));
    assert_eq!("প্ট", parse("pT"));
    assert_eq!("প্ত", parse("pt"));
    assert_eq!("প্ন", parse("pn"));
    assert_eq!("প্প", parse("pp"));
    assert_eq!("প্ল", parse("pl"));
    assert_eq!("প্স", parse("ps"));
    assert_eq!("ফ", parse("ph"));
    assert_eq!("ফ্ল", parse("fl"));
    assert_eq!("ফ", parse("f"));
    assert_eq!("প", parse("p"));
    assert_eq!("ৃ", parse("rri`"));
    assert_eq!("ঋ", parse("rri"));
    assert_eq!("কৃ", parse("krri"));
    assert_eq!("ঈঋ", parse("Irri"));
    assert_eq!("ঁঋ", parse("^rri"));
    assert_eq!("ঃঋ", parse(":rri"));
    assert_eq!("র‍্য", parse("rZ"));
}

#[test]
fn basic_test6() {
    assert_eq!("ক্র্য", parse("krZ"));
    assert_eq!("রর‍্য", parse("rrZ"));
    assert_eq!("ইয়র‍্য", parse("yrZ"));
    assert_eq!("ওর‍্য", parse("wrZ"));
    assert_eq!("এক্সর‍্য", parse("xrZ"));
    assert_eq!("ইর‍্য", parse("irZ"));
    assert_eq!("-র‍্য", parse("-rZ"));
    assert_eq!("ররর‍্য", parse("rrrZ"));
    assert_eq!("র‍্য", parse("ry"));
    assert_eq!("ক্র্য", parse("qry"));
    assert_eq!("রর‍্য", parse("rry"));
    assert_eq!("ইয়র‍্য", parse("yry"));
    assert_eq!("ওর‍্য", parse("wry"));
    assert_eq!("এক্সর‍্য", parse("xry"));
    assert_eq!("০র‍্য", parse("0ry"));
    assert_eq!("রররর‍্য", parse("rrrry"));
    assert_eq!("ড়্র্য", parse("Rry"));
    assert_eq!("রর", parse("rr"));
    assert_eq!("আরর", parse("arr"));
    assert_eq!("আর্ক", parse("arrk"));
    assert_eq!("আররা", parse("arra"));
    assert_eq!("আরর", parse("arr"));
    assert_eq!("আরর!", parse("arr!"));
    assert_eq!("ক্রর", parse("krr"));
    assert_eq!("ক্ররা", parse("krra"));
    assert_eq!("ড়্গ", parse("Rg"));
    assert_eq!("ঢ়", parse("Rh"));
    assert_eq!("ড়", parse("R"));
    assert_eq!("র", parse("r"));
    assert_eq!("অর", parse("or"));
    assert_eq!("ম্র", parse("mr"));
    assert_eq!("১র", parse("1r"));
    assert_eq!("+র", parse("+r"));
    assert_eq!("রর", parse("rr"));
    assert_eq!("ইয়র", parse("yr"));
    assert_eq!("ওর", parse("wr"));
    assert_eq!("এক্সর", parse("xr"));
    assert_eq!("য্র", parse("zr"));
    assert_eq!("ম্রি", parse("mri"));
}

#[test]
fn basic_test7() {
    assert_eq!("শ্ছ", parse("shch"));
    assert_eq!("ষ্ঠ", parse("ShTh"));
    assert_eq!("ষ্ফ", parse("Shph"));
    assert_eq!("শ্ছ", parse("Sch"));
    assert_eq!("স্ক্ল", parse("skl"));
    assert_eq!("স্খ", parse("skh"));
    assert_eq!("স্থ", parse("sth"));
    assert_eq!("স্ফ", parse("sph"));
    assert_eq!("শ্চ", parse("shc"));
    assert_eq!("শ্ত", parse("sht"));
    assert_eq!("শ্ন", parse("shn"));
    assert_eq!("শ্ম", parse("shm"));
    assert_eq!("শ্ল", parse("shl"));
    assert_eq!("ষ্ক", parse("Shk"));
    assert_eq!("ষ্ট", parse("ShT"));
    assert_eq!("ষ্ণ", parse("ShN"));
    assert_eq!("ষ্প", parse("Shp"));
    assert_eq!("ষ্ফ", parse("Shf"));
    assert_eq!("ষ্ম", parse("Shm"));
    assert_eq!("স্প্ল", parse("spl"));
    assert_eq!("স্ক", parse("sk"));
    assert_eq!("শ্চ", parse("Sc"));
    assert_eq!("স্ট", parse("sT"));
    assert_eq!("স্ত", parse("st"));
    assert_eq!("স্ন", parse("sn"));
    assert_eq!("স্প", parse("sp"));
    assert_eq!("স্ফ", parse("sf"));
    assert_eq!("স্ম", parse("sm"));
    assert_eq!("স্ল", parse("sl"));
    assert_eq!("শ", parse("sh"));
    assert_eq!("শ্চ", parse("Sc"));
    assert_eq!("শ্ত", parse("St"));
    assert_eq!("শ্ন", parse("Sn"));
    assert_eq!("শ্ম", parse("Sm"));
    assert_eq!("শ্ল", parse("Sl"));
    assert_eq!("ষ", parse("Sh"));
    assert_eq!("স", parse("s"));
    assert_eq!("শ", parse("S"));
    assert_eq!("উ", parse("oo"));
    assert_eq!("ওও", parse("OO"));
    assert_eq!("ু", parse("oo`"));
    assert_eq!("কু", parse("koo"));
    assert_eq!("উঅ", parse("ooo"));
    assert_eq!("!উ", parse("!oo"));
    assert_eq!("!উঅ", parse("!ooo"));
    assert_eq!("আউ", parse("aoo"));
    assert_eq!("উপ", parse("oop"));
    assert_eq!("উ", parse("ooo`"));
    assert_eq!("", parse("o`"));
    assert_eq!("অ্য", parse("oZ"));
    assert_eq!("অয়", parse("oY"));
    assert_eq!("অ", parse("o"));
    assert_eq!("!অ", parse("!o"));
    assert_eq!("ঁঅ", parse("^o"));
    assert_eq!("*অ", parse("*o"));
    assert_eq!("ইও", parse("io"));
    assert_eq!("ইয়", parse("yo"));
    assert_eq!("ন", parse("no"));
    assert_eq!("ত্থ", parse("tth"));
    assert_eq!("ৎ", parse("t``"));
    assert_eq!("ৎ", parse("`t``"));
    assert_eq!("ৎৎ", parse("t``t``"));
    assert_eq!("ৎ", parse("t```"));
    assert_eq!("ট্ট", parse("TT"));
    assert_eq!("ট্ম", parse("Tm"));
    assert_eq!("ঠ", parse("Th"));
    assert_eq!("ত্ন", parse("tn"));
    assert_eq!("ত্ম", parse("tm"));
    assert_eq!("থ", parse("th"));
    assert_eq!("ত্ত", parse("tt"));
    assert_eq!("ট", parse("T"));
    assert_eq!("ত", parse("t"));
}

#[test]
fn basic_test8() {
    assert_eq!("অ্যা", parse("aZ"));
    assert_eq!("আঅ্যা", parse("aaZ"));
    assert_eq!("অ্যা", parse("AZ"));
    assert_eq!("া", parse("a`"));
    assert_eq!("া", parse("a``"));
    assert_eq!("কা", parse("ka`"));
    assert_eq!("া", parse("A`"));
    assert_eq!("আ", parse("a"));
    assert_eq!("আ", parse("`a"));
    assert_eq!("কআ", parse("k`a"));
    assert_eq!("ইয়া", parse("ia"));
    assert_eq!("আআআা", parse("aaaa`"));
    assert_eq!("ি", parse("i`"));
    assert_eq!("ই", parse("i"));
    assert_eq!("ই", parse("`i"));
    assert_eq!("হি", parse("hi"));
    assert_eq!("ইহ", parse("ih"));
    assert_eq!("িহ", parse("i`h"));
    assert_eq!("ী", parse("I`"));
    assert_eq!("ঈ", parse("I"));
    assert_eq!("চী", parse("cI"));
    assert_eq!("ঈক্স", parse("Ix"));
    assert_eq!("ঈঈ", parse("II"));
    assert_eq!("০ঈ", parse("0I"));
    assert_eq!("অঈ", parse("oI"));
    assert_eq!("ু", parse("u`"));
    assert_eq!("উ", parse("u"));
    assert_eq!("কু", parse("ku"));
    assert_eq!("উক", parse("uk"));
    assert_eq!("উউ", parse("uu"));
    assert_eq!("ইউ", parse("iu"));
    assert_eq!("&উ", parse("&u"));
    assert_eq!("উ&", parse("u&"));
    assert_eq!("ূ", parse("U`"));
    assert_eq!("ঊ", parse("U"));
    assert_eq!("ইয়ূ", parse("yU"));
    assert_eq!("ঊয়", parse("Uy"));
    assert_eq!("ঁঊ", parse("^U"));
    assert_eq!("ঊঁ", parse("U^"));
    assert_eq!("ঈ", parse("EE"));
    assert_eq!("ঈ", parse("ee"));
    assert_eq!("ঈ", parse("Ee"));
    assert_eq!("ঈ", parse("eE"));
    assert_eq!("ী", parse("ee`"));
    assert_eq!("কী", parse("kee"));
    assert_eq!("ঈক", parse("eek"));
    assert_eq!("০ঈ", parse("0ee"));
    assert_eq!("ঈ৮", parse("ee8"));
    assert_eq!("(ঈ)", parse("(ee)"));
    assert_eq!("ে", parse("e`"));
    assert_eq!("এ", parse("e"));
    assert_eq!("কে", parse("ke"));
    assert_eq!("ওয়ে", parse("we"));
    assert_eq!("#এ#", parse("#e#"));
    assert_eq!("ে", parse("`e`"));
    assert_eq!("য", parse("z"));
    assert_eq!("্য", parse("Z"));
    assert_eq!("র‍্য", parse("rZ"));
    assert_eq!("ক্যশ", parse("kZS"));
    assert_eq!("ইয়", parse("y"));
    assert_eq!("অয়", parse("oy"));
    assert_eq!("ক্য", parse("ky"));
    assert_eq!("ইয়া", parse("ya"));
    assert_eq!("ইয়াআ", parse("yaa"));
    assert_eq!("য়", parse("Y"));
    assert_eq!("য়য়", parse("YY"));
    assert_eq!("ইয়", parse("iY"));
    assert_eq!("কয়", parse("kY"));
    assert_eq!("ক", parse("q"));
    assert_eq!("ক", parse("Q"));
    assert_eq!("ও", parse("w"));
    assert_eq!("ওয়া", parse("wa"));
    assert_eq!("-ওয়া-", parse("-wa-"));
    assert_eq!("ওয়ু", parse("woo"));
    assert_eq!("ওরে", parse("wre"));
    assert_eq!("ক্ব", parse("kw"));
    assert_eq!("এক্স", parse("x"));
    assert_eq!("এক্স", parse("ex"));
    assert_eq!("বক্স", parse("bx"));
    assert_eq!(":", parse(":`"));
    assert_eq!("ঃ", parse(":"));
    assert_eq!("^", parse("^`"));
    assert_eq!("ঁ", parse("^"));
    assert_eq!("কঁ", parse("k^"));
    assert_eq!("কঁই", parse("k^i"));
    assert_eq!("কিঁ", parse("ki^"));
    assert_eq!("্‌", parse(",,"));
    assert_eq!("্‌,", parse(",,,"));
    assert_eq!("্‌,", parse(",,`,"));
    assert_eq!("্‌", parse("`,,"));
    assert_eq!(",,", parse(",`,"));
    assert_eq!("৳", parse("$"));
    assert_eq!("", parse("`"));
    assert_eq!("ব্ধ", parse("bdh"));
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
