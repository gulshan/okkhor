use crate::models::{Pattern, Patterns};
use std::collections::HashMap;

const SHOR_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace("o", "অ"),
    Pattern::simple_replace("oo", "ঽ"),
    Pattern::simple_replace("fuf", "‌ু"),
    Pattern::simple_replace("fuuf", "‌ূ"),
    Pattern::simple_replace("fqf", "‌ৃ"),
    Pattern::simple_replace("fa", "া"),
    Pattern::simple_replace("a", "আ"),
    Pattern::simple_replace("fi", "ি"),
    Pattern::simple_replace("i", "ই"),
    Pattern::simple_replace("fii", "ী"),
    Pattern::simple_replace("ii", "ঈ"),
    Pattern::simple_replace("fu", "ু"),
    Pattern::simple_replace("u", "উ"),
    Pattern::simple_replace("fuu", "ূ"),
    Pattern::simple_replace("uu", "ঊ"),
    Pattern::simple_replace("fq", "ৃ"),
    Pattern::simple_replace("q", "ঋ"),
    Pattern::simple_replace("fe", "ে"),
    Pattern::simple_replace("e", "এ"),
    Pattern::simple_replace("foi", "ৈ"),
    Pattern::simple_replace("oi", "ঐ"),
    Pattern::simple_replace("fw", "ো"),
    Pattern::simple_replace("w", "ও"),
    Pattern::simple_replace("fou", "ৌ"),
    Pattern::simple_replace("ou", "ঔ"),
    Pattern::simple_replace("fae", "্যা"),
    Pattern::simple_replace("ae", "অ্যা"),
    Pattern::simple_replace("wa", "ওয়া"),
    Pattern::simple_replace("fwa", "োয়া"),
    Pattern::simple_replace("wae", "ওয়্যা"),
    Pattern::simple_replace("we", "ওয়ে"),
    Pattern::simple_replace("fwe", "োয়ে"),
    Pattern::simple_replace("ngo", "ঙ"),
    Pattern::simple_replace("nga", "ঙা"),
    Pattern::simple_replace("ngi", "ঙি"),
    Pattern::simple_replace("ngii", "ঙী"),
    Pattern::simple_replace("ngu", "ঙু"),
    Pattern::simple_replace("nguff", "ঙ"),
    Pattern::simple_replace("nguu", "ঙূ"),
    Pattern::simple_replace("nguuff", "ঙ"),
    Pattern::simple_replace("ngq", "ঙৃ"),
    Pattern::simple_replace("nge", "ঙে"),
    Pattern::simple_replace("ngoi", "ঙৈ"),
    Pattern::simple_replace("ngw", "ঙো"),
    Pattern::simple_replace("ngou", "ঙৌ"),
    Pattern::simple_replace("ngae", "ঙ্যা"),
];

const BYANJON_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace("k", "ক"),
    Pattern::simple_replace("kh", "খ"),
    Pattern::simple_replace("g", "গ"),
    Pattern::simple_replace("gh", "ঘ"),
    Pattern::simple_replace("c", "চ"),
    Pattern::simple_replace("ch", "ছ"),
    Pattern::simple_replace("j", "জ"),
    Pattern::simple_replace("jh", "ঝ"),
    Pattern::simple_replace("nff", "ঞ"),
    Pattern::simple_replace("tf", "ট"),
    Pattern::simple_replace("tff", "ঠ"),
    Pattern::simple_replace("tfh", "ঠ"),
    Pattern::simple_replace("df", "ড"),
    Pattern::simple_replace("dff", "ঢ"),
    Pattern::simple_replace("dfh", "ঢ"),
    Pattern::simple_replace("nf", "ণ"),
    Pattern::simple_replace("t", "ত"),
    Pattern::simple_replace("th", "থ"),
    Pattern::simple_replace("d", "দ"),
    Pattern::simple_replace("dh", "ধ"),
    Pattern::simple_replace("n", "ন"),
    Pattern::simple_replace("p", "প"),
    Pattern::simple_replace("ph", "ফ"),
    Pattern::simple_replace("b", "ব"),
    Pattern::simple_replace("v", "ভ"),
    Pattern::simple_replace("m", "ম"),
    Pattern::simple_replace("z", "য"),
    Pattern::simple_replace("l", "ল"),
    Pattern::simple_replace("sh", "শ"),
    Pattern::simple_replace("sf", "ষ"),
    Pattern::simple_replace("s", "স"),
    Pattern::simple_replace("h", "হ"),
    Pattern::simple_replace("y", "য়"),
    Pattern::simple_replace("rf", "ড়"),
    Pattern::simple_replace("rff", "ঢ়"),
    Pattern::simple_replace(",,", "়"),
];

const JUKTOBORNO_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace("rz", "র‍্য"),
    Pattern::simple_replace("kk", "ক্ক"),
    Pattern::simple_replace("ktf", "ক্ট"),
    Pattern::simple_replace("ktfr", "ক্ট্র"),
    Pattern::simple_replace("kt", "ক্ত"),
    Pattern::simple_replace("ktr", "ক্ত্র"),
    Pattern::simple_replace("kb", "ক্ব"),
    Pattern::simple_replace("km", "ক্ম"),
    Pattern::simple_replace("kz", "ক্য"),
    Pattern::simple_replace("kr", "ক্র"),
    Pattern::simple_replace("kl", "ক্ল"),
    Pattern::simple_replace("kf", "ক্ষ"),
    Pattern::simple_replace("ksf", "ক্ষ"),
    Pattern::simple_replace("kkh", "ক্ষ"),
    Pattern::simple_replace("kfnf", "ক্ষ্ণ"),
    Pattern::simple_replace("kfn", "ক্ষ্ণ"),
    Pattern::simple_replace("ksfnf", "ক্ষ্ণ"),
    Pattern::simple_replace("ksfn", "ক্ষ্ণ"),
    Pattern::simple_replace("kkhn", "ক্ষ্ণ"),
    Pattern::simple_replace("kkhnf", "ক্ষ্ণ"),
    Pattern::simple_replace("kfb", "ক্ষ্ব"),
    Pattern::simple_replace("ksfb", "ক্ষ্ব"),
    Pattern::simple_replace("kkhb", "ক্ষ্ব"),
    Pattern::simple_replace("kfm", "ক্ষ্ম"),
    Pattern::simple_replace("kkhm", "ক্ষ্ম"),
    Pattern::simple_replace("ksfm", "ক্ষ্ম"),
    Pattern::simple_replace("kfz", "ক্ষ্য"),
    Pattern::simple_replace("ksfz", "ক্ষ্য"),
    Pattern::simple_replace("kkhz", "ক্ষ্য"),
    Pattern::simple_replace("ks", "ক্স"),
    Pattern::simple_replace("khz", "খ্য"),
    Pattern::simple_replace("khr", "খ্র"),
    Pattern::simple_replace("ggg", "গ্গ"),
    Pattern::simple_replace("gnf", "গ্‌ণ"),
    Pattern::simple_replace("gdh", "গ্ধ"),
    Pattern::simple_replace("gdhz", "গ্ধ্য"),
    Pattern::simple_replace("gdhr", "গ্ধ্র"),
    Pattern::simple_replace("gn", "গ্ন"),
    Pattern::simple_replace("gnz", "গ্ন্য"),
    Pattern::simple_replace("gb", "গ্ব"),
    Pattern::simple_replace("gm", "গ্ম"),
    Pattern::simple_replace("gz", "গ্য"),
    Pattern::simple_replace("gr", "গ্র"),
    Pattern::simple_replace("grz", "গ্র্য"),
    Pattern::simple_replace("gl", "গ্ল"),
    Pattern::simple_replace("ghn", "ঘ্ন"),
    Pattern::simple_replace("ghr", "ঘ্র"),
    Pattern::simple_replace("ngk", "ঙ্ক"),
    Pattern::simple_replace("ngkt", "ঙ্‌ক্ত"),
    Pattern::simple_replace("ngkz", "ঙ্ক্য"),
    Pattern::simple_replace("ngkr", "ঙ্ক্র"),
    Pattern::simple_replace("ngkkh", "ঙ্ক্ষ"),
    Pattern::simple_replace("ngksf", "ঙ্ক্ষ"),
    Pattern::simple_replace("ngkh", "ঙ্খ"),
    Pattern::simple_replace("ngg", "ঙ্গ"),
    Pattern::simple_replace("nggz", "ঙ্গ্য"),
    Pattern::simple_replace("nggh", "ঙ্ঘ"),
    Pattern::simple_replace("ngghz", "ঙ্ঘ্য"),
    Pattern::simple_replace("ngghr", "ঙ্ঘ্র"),
    Pattern::simple_replace("ngm", "ঙ্ম"),
    Pattern::simple_replace("cc", "চ্চ"),
    Pattern::simple_replace("cch", "চ্ছ"),
    Pattern::simple_replace("cchb", "চ্ছ্ব"),
    Pattern::simple_replace("cchr", "চ্ছ্র"),
    Pattern::simple_replace("cnff", "চ্ঞ"),
    Pattern::simple_replace("cb", "চ্ব"),
    Pattern::simple_replace("cz", "চ্য"),
    Pattern::simple_replace("jj", "জ্জ"),
    Pattern::simple_replace("jjb", "জ্জ্ব"),
    Pattern::simple_replace("jjh", "জ্ঝ"),
    Pattern::simple_replace("jnff", "জ্ঞ"),
    Pattern::simple_replace("gg", "জ্ঞ"),
    Pattern::simple_replace("jb", "জ্ব"),
    Pattern::simple_replace("jz", "জ্য"),
    Pattern::simple_replace("jr", "জ্র"),
    Pattern::simple_replace("nc", "ঞ্চ"),
    Pattern::simple_replace("nffc", "ঞ্চ"),
    Pattern::simple_replace("nj", "ঞ্জ"),
    Pattern::simple_replace("nffj", "ঞ্জ"),
    Pattern::simple_replace("njh", "ঞ্ঝ"),
    Pattern::simple_replace("nffjh", "ঞ্ঝ"),
    Pattern::simple_replace("nch", "ঞ্ছ"),
    Pattern::simple_replace("nffch", "ঞ্ছ"),
    Pattern::simple_replace("ttf", "ট্ট"),
    Pattern::simple_replace("tftf", "ট্ট"),
    Pattern::simple_replace("tfb", "ট্ব"),
    Pattern::simple_replace("tfm", "ট্ম"),
    Pattern::simple_replace("tfz", "ট্য"),
    Pattern::simple_replace("tfr", "ট্র"),
    Pattern::simple_replace("ddf", "ড্ড"),
    Pattern::simple_replace("dfdf", "ড্ড"),
    Pattern::simple_replace("dfb", "ড্ব"),
    Pattern::simple_replace("dfz", "ড্য"),
    Pattern::simple_replace("dfr", "ড্র"),
    Pattern::simple_replace("rfg", "ড়্‌গ"),
    Pattern::simple_replace("dffz", "ঢ্য"),
    Pattern::simple_replace("dfhz", "ঢ্য"),
    Pattern::simple_replace("dffr", "ঢ্র"),
    Pattern::simple_replace("dfhr", "ঢ্র"),
    Pattern::simple_replace("nftf", "ণ্ট"),
    Pattern::simple_replace("nftff", "ণ্ঠ"),
    Pattern::simple_replace("nftfh", "ণ্ঠ"),
    Pattern::simple_replace("nftffz", "ণ্ঠ্য"),
    Pattern::simple_replace("nftfhz", "ণ্ঠ্য"),
    Pattern::simple_replace("nfdf", "ণ্ড"),
    Pattern::simple_replace("nfdfz", "ণ্ড্য"),
    Pattern::simple_replace("nfdfr", "ণ্ড্র"),
    Pattern::simple_replace("nfdff", "ণ্ঢ"),
    Pattern::simple_replace("nfdfh", "ণ্ঢ"),
    Pattern::simple_replace("nfnf", "ণ্ণ"),
    Pattern::simple_replace("nfn", "ণ্ণ"),
    Pattern::simple_replace("nfb", "ণ্ব"),
    Pattern::simple_replace("nfm", "ণ্ম"),
    Pattern::simple_replace("nfz", "ণ্য"),
    Pattern::simple_replace("tt", "ত্ত"),
    Pattern::simple_replace("ttb", "ত্ত্ব"),
    Pattern::simple_replace("ttz", "ত্ত্য"),
    Pattern::simple_replace("tth", "ত্থ"),
    Pattern::simple_replace("tn", "ত্ন"),
    Pattern::simple_replace("tb", "ত্ব"),
    Pattern::simple_replace("tm", "ত্ম"),
    Pattern::simple_replace("tmz", "ত্ম্য"),
    Pattern::simple_replace("tz", "ত্য"),
    Pattern::simple_replace("tr", "ত্র"),
    Pattern::simple_replace("trz", "ত্র্য"),
    Pattern::simple_replace("thb", "থ্ব"),
    Pattern::simple_replace("thz", "থ্য"),
    Pattern::simple_replace("thr", "থ্র"),
    Pattern::simple_replace("dg", "দ্‌গ"),
    Pattern::simple_replace("dgh", "দ্‌ঘ"),
    Pattern::simple_replace("dd", "দ্দ"),
    Pattern::simple_replace("ddb", "দ্দ্ব"),
    Pattern::simple_replace("ddh", "দ্ধ"),
    Pattern::simple_replace("db", "দ্ব"),
    Pattern::simple_replace("dv", "দ্ভ"),
    Pattern::simple_replace("dvr", "দ্ভ্র"),
    Pattern::simple_replace("dm", "দ্ম"),
    Pattern::simple_replace("dz", "দ্য"),
    Pattern::simple_replace("dr", "দ্র"),
    Pattern::simple_replace("drz", "দ্র্য"),
    Pattern::simple_replace("dhn", "ধ্ন"),
    Pattern::simple_replace("dhb", "ধ্ব"),
    Pattern::simple_replace("dhm", "ধ্ম"),
    Pattern::simple_replace("dhz", "ধ্য"),
    Pattern::simple_replace("dhr", "ধ্র"),
    Pattern::simple_replace("ntf", "ন্ট"),
    Pattern::simple_replace("ntfr", "ন্ট্র"),
    Pattern::simple_replace("ntff", "ন্ঠ"),
    Pattern::simple_replace("ntfh", "ন্ঠ"),
    Pattern::simple_replace("ndf", "ন্ড"),
    Pattern::simple_replace("ndfr", "ন্ড্র"),
    Pattern::simple_replace("nt", "ন্ত"),
    Pattern::simple_replace("ntb", "ন্ত্ব"),
    Pattern::simple_replace("ntr", "ন্ত্র"),
    Pattern::simple_replace("ntrz", "ন্ত্র্য"),
    Pattern::simple_replace("nth", "ন্থ"),
    Pattern::simple_replace("nthr", "ন্থ্র"),
    Pattern::simple_replace("nd", "ন্দ"),
    Pattern::simple_replace("ndb", "ন্দ্ব"),
    Pattern::simple_replace("ndz", "ন্দ্য"),
    Pattern::simple_replace("ndr", "ন্দ্র"),
    Pattern::simple_replace("ndh", "ন্ধ"),
    Pattern::simple_replace("ndhz", "ন্ধ্য"),
    Pattern::simple_replace("ndhr", "ন্ধ্র"),
    Pattern::simple_replace("nn", "ন্ন"),
    Pattern::simple_replace("nb", "ন্ব"),
    Pattern::simple_replace("nm", "ন্ম"),
    Pattern::simple_replace("nz", "ন্য"),
    Pattern::simple_replace("ns", "ন্স"),
    Pattern::simple_replace("ptf", "প্ট"),
    Pattern::simple_replace("pt", "প্ত"),
    Pattern::simple_replace("pn", "প্ন"),
    Pattern::simple_replace("pp", "প্প"),
    Pattern::simple_replace("pz", "প্য"),
    Pattern::simple_replace("pr", "প্র"),
    Pattern::simple_replace("pl", "প্ল"),
    Pattern::simple_replace("ps", "প্স"),
    Pattern::simple_replace("phr", "ফ্র"),
    Pattern::simple_replace("phl", "ফ্ল"),
    Pattern::simple_replace("bj", "ব্জ"),
    Pattern::simple_replace("bd", "ব্দ"),
    Pattern::simple_replace("bdh", "ব্ধ"),
    Pattern::simple_replace("bb", "ব্ব"),
    Pattern::simple_replace("bz", "ব্য"),
    Pattern::simple_replace("br", "ব্র"),
    Pattern::simple_replace("bl", "ব্ল"),
    Pattern::simple_replace("vb", "ভ্ব"),
    Pattern::simple_replace("vz", "ভ্য"),
    Pattern::simple_replace("vr", "ভ্র"),
    Pattern::simple_replace("vl", "ভ্ল"),
    Pattern::simple_replace("mn", "ম্ন"),
    Pattern::simple_replace("mp", "ম্প"),
    Pattern::simple_replace("mpr", "ম্প্র"),
    Pattern::simple_replace("mph", "ম্ফ"),
    Pattern::simple_replace("mb", "ম্ব"),
    Pattern::simple_replace("mbr", "ম্ব্র"),
    Pattern::simple_replace("mv", "ম্ভ"),
    Pattern::simple_replace("mvr", "ম্ভ্র"),
    Pattern::simple_replace("mm", "ম্ম"),
    Pattern::simple_replace("mz", "ম্য"),
    Pattern::simple_replace("mr", "ম্র"),
    Pattern::simple_replace("ml", "ম্ল"),
    Pattern::simple_replace("zz", "য্য"),
    Pattern::simple_replace("lk", "ল্ক"),
    Pattern::simple_replace("lkz", "ল্ক্য"),
    Pattern::simple_replace("lg", "ল্গ"),
    Pattern::simple_replace("ltf", "ল্ট"),
    Pattern::simple_replace("ldf", "ল্ড"),
    Pattern::simple_replace("lp", "ল্প"),
    Pattern::simple_replace("lph", "ল্ফ"),
    Pattern::simple_replace("lb", "ল্ব"),
    Pattern::simple_replace("lv", "ল্‌ভ"),
    Pattern::simple_replace("lm", "ল্ম"),
    Pattern::simple_replace("lz", "ল্য"),
    Pattern::simple_replace("ll", "ল্ল"),
    Pattern::simple_replace("shc", "শ্চ"),
    Pattern::simple_replace("shch", "শ্ছ"),
    Pattern::simple_replace("shn", "শ্ন"),
    Pattern::simple_replace("shb", "শ্ব"),
    Pattern::simple_replace("shm", "শ্ম"),
    Pattern::simple_replace("shz", "শ্য"),
    Pattern::simple_replace("shr", "শ্র"),
    Pattern::simple_replace("shl", "শ্ল"),
    Pattern::simple_replace("sfk", "ষ্ক"),
    Pattern::simple_replace("sfkr", "ষ্ক্র"),
    Pattern::simple_replace("sftf", "ষ্ট"),
    Pattern::simple_replace("sftfz", "ষ্ট্য"),
    Pattern::simple_replace("sftfr", "ষ্ট্র"),
    Pattern::simple_replace("sftff", "ষ্ঠ"),
    Pattern::simple_replace("sftfh", "ষ্ঠ"),
    Pattern::simple_replace("sftffz", "ষ্ঠ্য"),
    Pattern::simple_replace("sftfhz", "ষ্ঠ্য"),
    Pattern::simple_replace("sfnf", "ষ্ণ"),
    Pattern::simple_replace("sfn", "ষ্ণ"),
    Pattern::simple_replace("sfp", "ষ্প"),
    Pattern::simple_replace("sfpr", "ষ্প্র"),
    Pattern::simple_replace("sfph", "ষ্ফ"),
    Pattern::simple_replace("sfb", "ষ্ব"),
    Pattern::simple_replace("sfm", "ষ্ম"),
    Pattern::simple_replace("sfz", "ষ্য"),
    Pattern::simple_replace("sk", "স্ক"),
    Pattern::simple_replace("skr", "স্ক্র"),
    Pattern::simple_replace("skh", "স্খ"),
    Pattern::simple_replace("stf", "স্ট"),
    Pattern::simple_replace("stfr", "স্ট্র"),
    Pattern::simple_replace("st", "স্ত"),
    Pattern::simple_replace("stb", "স্ত্ব"),
    Pattern::simple_replace("stz", "স্ত্য"),
    Pattern::simple_replace("str", "স্ত্র"),
    Pattern::simple_replace("sth", "স্থ"),
    Pattern::simple_replace("sthz", "স্থ্য"),
    Pattern::simple_replace("sn", "স্ন"),
    Pattern::simple_replace("sp", "স্প"),
    Pattern::simple_replace("spr", "স্প্র"),
    Pattern::simple_replace("spl", "স্প্ল"),
    Pattern::simple_replace("sph", "স্ফ"),
    Pattern::simple_replace("sb", "স্ব"),
    Pattern::simple_replace("sm", "স্ম"),
    Pattern::simple_replace("sz", "স্য"),
    Pattern::simple_replace("sr", "স্র"),
    Pattern::simple_replace("sl", "স্ল"),
    Pattern::simple_replace("hn", "হ্ন"),
    Pattern::simple_replace("hnf", "হ্ণ"),
    Pattern::simple_replace("hb", "হ্ব"),
    Pattern::simple_replace("hm", "হ্ম"),
    Pattern::simple_replace("hz", "হ্য"),
    Pattern::simple_replace("hr", "হ্র"),
    Pattern::simple_replace("hl", "হ্ল"),
    // oshomvob juktoborno
    Pattern::simple_replace("ksh", "কশ"),
    Pattern::simple_replace("nsh", "নশ"),
    Pattern::simple_replace("psh", "পশ"),
    Pattern::simple_replace("ld", "লদ"),
    Pattern::simple_replace("gd", "গদ"),
    Pattern::simple_replace("ngkk", "ঙ্কক"),
    Pattern::simple_replace("ngks", "ঙ্কস"),
    Pattern::simple_replace("cn", "চন"),
    Pattern::simple_replace("cnf", "চণ"),
    Pattern::simple_replace("jn", "জন"),
    Pattern::simple_replace("jnf", "জণ"),
    Pattern::simple_replace("tft", "টত"),
    Pattern::simple_replace("dfd", "ডদ"),
    Pattern::simple_replace("nft", "ণত"),
    Pattern::simple_replace("nfd", "ণদ"),
    Pattern::simple_replace("lt", "লত"),
    Pattern::simple_replace("sft", "ষত"),
    Pattern::simple_replace("nfth", "ণথ"),
    Pattern::simple_replace("nfdh", "ণধ"),
    Pattern::simple_replace("sfth", "ষথ"),
    Pattern::simple_replace("ktff", "কঠ"),
    Pattern::simple_replace("ktfh", "কঠ"),
    Pattern::simple_replace("ptff", "পঠ"),
    Pattern::simple_replace("ptfh", "পঠ"),
    Pattern::simple_replace("ltff", "লঠ"),
    Pattern::simple_replace("ltfh", "লঠ"),
    Pattern::simple_replace("stff", "সঠ"),
    Pattern::simple_replace("stfh", "সঠ"),
    Pattern::simple_replace("dfdff", "ডঢ"),
    Pattern::simple_replace("dfdfh", "ডঢ"),
    Pattern::simple_replace("ndff", "নঢ"),
    Pattern::simple_replace("ndfh", "নঢ"),
    Pattern::simple_replace("ktfrf", "ক্টড়"),
    Pattern::simple_replace("ktfrff", "ক্টঢ়"),
    Pattern::simple_replace("kth", "কথ"),
    Pattern::simple_replace("ktrf", "ক্তড়"),
    Pattern::simple_replace("ktrff", "ক্তঢ়"),
    Pattern::simple_replace("krf", "কড়"),
    Pattern::simple_replace("krff", "কঢ়"),
    Pattern::simple_replace("khrf", "খড়"),
    Pattern::simple_replace("khrff", "খঢ়"),
    Pattern::simple_replace("gggh", "জ্ঞঘ"),
    Pattern::simple_replace("gdff", "গঢ"),
    Pattern::simple_replace("gdfh", "গঢ"),
    Pattern::simple_replace("gdhrf", "গ্ধড়"),
    Pattern::simple_replace("gdhrff", "গ্ধঢ়"),
    Pattern::simple_replace("grf", "গড়"),
    Pattern::simple_replace("grff", "গঢ়"),
    Pattern::simple_replace("ghrf", "ঘড়"),
    Pattern::simple_replace("ghrff", "ঘঢ়"),
    Pattern::simple_replace("ngkth", "ঙ্কথ"),
    Pattern::simple_replace("ngkrf", "ঙ্কড়"),
    Pattern::simple_replace("ngkrff", "ঙ্কঢ়"),
    Pattern::simple_replace("ngghrf", "ঙ্ঘড়"),
    Pattern::simple_replace("ngghrff", "ঙ্ঘঢ়"),
    Pattern::simple_replace("cchrf", "চ্ছড়"),
    Pattern::simple_replace("cchrff", "চ্ছঢ়"),
    Pattern::simple_replace("tfrf", "টড়"),
    Pattern::simple_replace("tfrff", "টঢ়"),
    Pattern::simple_replace("dfrf", "ডড়"),
    Pattern::simple_replace("dfrff", "ডঢ়"),
    Pattern::simple_replace("rfgh", "ড়ঘ"),
    Pattern::simple_replace("dffrf", "ঢড়"),
    Pattern::simple_replace("dfhrf", "ঢড়"),
    Pattern::simple_replace("dffrff", "ঢঢ়"),
    Pattern::simple_replace("dfhrff", "ঢঢ়"),
    Pattern::simple_replace("nfdfrf", "ণ্ডড়"),
    Pattern::simple_replace("nfdfrff", "ণ্ডঢ়"),
    Pattern::simple_replace("trf", "তড়"),
    Pattern::simple_replace("trff", "তঢ়"),
    Pattern::simple_replace("thrf", "থড়"),
    Pattern::simple_replace("thrff", "থঢ়"),
    Pattern::simple_replace("dvrf", "দ্ভড়"),
    Pattern::simple_replace("dvrff", "দ্ভঢ়"),
    Pattern::simple_replace("drf", "দড়"),
    Pattern::simple_replace("drff", "দঢ়"),
    Pattern::simple_replace("dhrf", "ধড়"),
    Pattern::simple_replace("dhrff", "ধঢ়"),
    Pattern::simple_replace("ntfrf", "ন্টড়"),
    Pattern::simple_replace("ntfrff", "ন্টঢ়"),
    Pattern::simple_replace("ndfrf", "ন্ডড়"),
    Pattern::simple_replace("ndfrff", "ন্ডঢ়"),
    Pattern::simple_replace("ntrf", "ন্তড়"),
    Pattern::simple_replace("ntrff", "ন্তঢ়"),
    Pattern::simple_replace("nthrf", "ন্থড়"),
    Pattern::simple_replace("nthrff", "ন্থঢ়"),
    Pattern::simple_replace("ndrf", "ন্দড়"),
    Pattern::simple_replace("ndrff", "ন্দঢ়"),
    Pattern::simple_replace("ndhrf", "ন্ধড়"),
    Pattern::simple_replace("ndhrff", "ন্ধঢ়"),
    Pattern::simple_replace("pth", "পথ"),
    Pattern::simple_replace("pph", "পফ"),
    Pattern::simple_replace("prf", "পড়"),
    Pattern::simple_replace("prff", "পঢ়"),
    Pattern::simple_replace("phrf", "ফড়"),
    Pattern::simple_replace("phrff", "ফঢ়"),
    Pattern::simple_replace("bjh", "বঝ"),
    Pattern::simple_replace("brf", "বড়"),
    Pattern::simple_replace("brff", "বঢ়"),
    Pattern::simple_replace("vrf", "ভড়"),
    Pattern::simple_replace("vrff", "ভঢ়"),
    Pattern::simple_replace("mprf", "ম্পড়"),
    Pattern::simple_replace("mprff", "ম্পঢ়"),
    Pattern::simple_replace("mbrf", "ম্বড়"),
    Pattern::simple_replace("mbrff", "ম্বঢ়"),
    Pattern::simple_replace("mvrf", "ম্ভড়"),
    Pattern::simple_replace("mvrff", "ম্ভঢ়"),
    Pattern::simple_replace("mrf", "মড়"),
    Pattern::simple_replace("mrff", "মঢ়"),
    Pattern::simple_replace("lkh", "লখ"),
    Pattern::simple_replace("lgh", "লঘ"),
    Pattern::simple_replace("shrf", "শড়"),
    Pattern::simple_replace("shrff", "শঢ়"),
    Pattern::simple_replace("sfkh", "ষখ"),
    Pattern::simple_replace("sfkrf", "ষ্কড়"),
    Pattern::simple_replace("sfkrff", "ষ্কঢ়"),
    Pattern::simple_replace("sftfrf", "ষ্টড়"),
    Pattern::simple_replace("sftfrff", "ষ্টঢ়"),
    Pattern::simple_replace("sfprf", "ষ্পড়"),
    Pattern::simple_replace("sfprff", "ষ্পঢ়"),
    Pattern::simple_replace("skrf", "স্কড়"),
    Pattern::simple_replace("skrff", "স্কঢ়"),
    Pattern::simple_replace("stfrf", "স্টড়"),
    Pattern::simple_replace("stfrff", "স্টঢ়"),
    Pattern::simple_replace("strf", "স্তড়"),
    Pattern::simple_replace("strff", "স্তঢ়"),
    Pattern::simple_replace("sprf", "স্পড়"),
    Pattern::simple_replace("sprff", "স্পঢ়"),
    Pattern::simple_replace("srf", "সড়"),
    Pattern::simple_replace("srff", "সঢ়"),
    Pattern::simple_replace("hrf", "হড়"),
    Pattern::simple_replace("hrff", "হঢ়"),
    Pattern::simple_replace("ldh", "লধ"),
    Pattern::simple_replace("ngksh", "ঙ্কশ"),
    Pattern::simple_replace("tfth", "টথ"),
    Pattern::simple_replace("dfdh", "ডধ"),
    Pattern::simple_replace("lth", "লথ"),
];

const REPH_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace("rr", "র্"),
    Pattern::simple_replace("r", "র"),
];

const PHOLA_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace("r", "র"),
    Pattern::simple_replace("z", "য"),
];

const KAR_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace("o", ""),
    Pattern::simple_replace("of", "অ"),
    Pattern::simple_replace("a", "া"),
    Pattern::simple_replace("af", "আ"),
    Pattern::simple_replace("i", "ি"),
    Pattern::simple_replace("if", "ই"),
    Pattern::simple_replace("ii", "ী"),
    Pattern::simple_replace("iif", "ঈ"),
    Pattern::simple_replace("u", "ু"),
    Pattern::simple_replace("uf", "উ"),
    Pattern::simple_replace("uu", "ূ"),
    Pattern::simple_replace("uuf", "ঊ"),
    Pattern::simple_replace("q", "ৃ"),
    Pattern::simple_replace("qf", "ঋ"),
    Pattern::simple_replace("e", "ে"),
    Pattern::simple_replace("ef", "এ"),
    Pattern::simple_replace("oi", "ৈ"),
    Pattern::simple_replace("oif", "ই"),
    Pattern::simple_replace("w", "ো"),
    Pattern::simple_replace("wf", "ও"),
    Pattern::simple_replace("ou", "ৌ"),
    Pattern::simple_replace("ouf", "উ"),
    Pattern::simple_replace("ae", "্যা"),
    Pattern::simple_replace("aef", "অ্যা"),
    Pattern::simple_replace("uff", "‌ু"),
    Pattern::simple_replace("uuff", "‌ূ"),
    Pattern::simple_replace("qff", "‌ৃ"),
    Pattern::simple_replace("we", "োয়ে"),
    Pattern::simple_replace("wef", "ওয়ে"),
    Pattern::simple_replace("waf", "ওয়া"),
    Pattern::simple_replace("wa", "োয়া"),
    Pattern::simple_replace("wae", "ওয়্যা"),
];

const ONGKO_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace(".1", ".১"),
    Pattern::simple_replace(".2", ".২"),
    Pattern::simple_replace(".3", ".৩"),
    Pattern::simple_replace(".4", ".৪"),
    Pattern::simple_replace(".5", ".৫"),
    Pattern::simple_replace(".6", ".৬"),
    Pattern::simple_replace(".7", ".৭"),
    Pattern::simple_replace(".8", ".৮"),
    Pattern::simple_replace(".9", ".৯"),
    Pattern::simple_replace(".0", ".০"),
    Pattern::simple_replace("1", "১"),
    Pattern::simple_replace("2", "২"),
    Pattern::simple_replace("3", "৩"),
    Pattern::simple_replace("4", "৪"),
    Pattern::simple_replace("5", "৫"),
    Pattern::simple_replace("6", "৬"),
    Pattern::simple_replace("7", "৭"),
    Pattern::simple_replace("8", "৮"),
    Pattern::simple_replace("9", "৯"),
    Pattern::simple_replace("0", "০"),
];

const DIACRITIC_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace("qq", "্"),
    Pattern::simple_replace("xx", "্‌"),
    Pattern::simple_replace("t/", "ৎ"),
    Pattern::simple_replace("x", "ঃ"),
    Pattern::simple_replace("ng", "ং"),
    Pattern::simple_replace("ngf", "ং"),
    Pattern::simple_replace("/", "ঁ"),
    Pattern::simple_replace("//", "/"),
    Pattern::simple_replace("`", "`"),
    Pattern::simple_replace("``", "‌"),
    Pattern::simple_replace("```", "``"),
    Pattern::simple_replace("~", "~"),
    Pattern::simple_replace("~~", "‍"),
    Pattern::simple_replace("~~~", "~~"),
];

const BIRAM_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace(".", "।"),
    Pattern::simple_replace("...", "..."),
    Pattern::simple_replace("..", "."),
    Pattern::simple_replace("$", "৳"),
    Pattern::simple_replace("$f", "₹"),
    Pattern::simple_replace(",,,", ",,"),
    Pattern::simple_replace(".f", "॥"),
    Pattern::simple_replace(".ff", "৺"),
    Pattern::simple_replace("+", "+"),
    Pattern::simple_replace("-", "-"),
    Pattern::simple_replace("+f", "×"),
    Pattern::simple_replace("-f", "÷"),
];

const PRITHAYOK_PHONETIC_PATTERNS: &[Pattern] = &[
    Pattern::simple_replace(";", ""),
    Pattern::simple_replace(";;", ";"),
];

const AE_PHONETIC_PATTERNS: &[Pattern] = &[Pattern::simple_replace("ae", "‍্যা")];

#[derive(PartialEq, Eq, Hash)]
enum Group {
    Shor,
    Byanjon,
    Juktoborno,
    Reph,
    Phola,
    Kar,
    Ongko,
    Diacritic,
    Biram,
    Prithayok,
    Ae,
}

#[derive(PartialEq)]
enum State {
    Init,
    Shor,
    Reph,
    Byanjon,
}

impl State {
    const fn groups(&self) -> &[Group] {
        match self {
            State::Init => &[
                Group::Diacritic,
                Group::Shor,
                Group::Prithayok,
                Group::Ongko,
                Group::Biram,
                Group::Reph,
                Group::Juktoborno,
                Group::Byanjon,
            ],
            State::Shor => &[
                Group::Diacritic,
                Group::Shor,
                Group::Biram,
                Group::Prithayok,
                Group::Ongko,
                Group::Reph,
                Group::Juktoborno,
                Group::Byanjon,
            ],
            State::Reph => &[
                Group::Prithayok,
                Group::Ae,
                Group::Juktoborno,
                Group::Byanjon,
                Group::Kar,
            ],
            State::Byanjon => &[
                Group::Diacritic,
                Group::Prithayok,
                Group::Ongko,
                Group::Biram,
                Group::Kar,
                Group::Juktoborno,
                Group::Phola,
                Group::Byanjon,
            ],
        }
    }

    const fn next_state(&self, group: &Group) -> State {
        match self {
            State::Init => match group {
                Group::Diacritic | Group::Shor => State::Shor,
                Group::Reph => State::Reph,
                Group::Juktoborno | Group::Byanjon => State::Byanjon,
                _ => State::Init,
            },
            State::Shor => match group {
                Group::Biram | Group::Prithayok | Group::Ongko => State::Init,
                Group::Reph => State::Reph,
                Group::Juktoborno | Group::Byanjon => State::Byanjon,
                _ => State::Shor,
            },
            State::Reph => match group {
                Group::Prithayok => State::Init,
                Group::Ae => State::Shor,
                Group::Juktoborno | Group::Byanjon => State::Byanjon,
                Group::Kar => State::Shor,
                _ => State::Reph,
            },
            State::Byanjon => match group {
                Group::Diacritic | Group::Kar => State::Shor,
                Group::Prithayok | Group::Ongko | Group::Biram => State::Init,
                _ => State::Byanjon,
            },
        }
    }
}

pub struct KhiproPhonetic {
    groups: HashMap<Group, Patterns>,
}

impl KhiproPhonetic {
    pub fn new() -> Self {
        let groups = [
            (Group::Shor, Patterns::new(SHOR_PHONETIC_PATTERNS)),
            (Group::Byanjon, Patterns::new(BYANJON_PHONETIC_PATTERNS)),
            (
                Group::Juktoborno,
                Patterns::new(JUKTOBORNO_PHONETIC_PATTERNS),
            ),
            (Group::Reph, Patterns::new(REPH_PHONETIC_PATTERNS)),
            (Group::Phola, Patterns::new(PHOLA_PHONETIC_PATTERNS)),
            (Group::Kar, Patterns::new(KAR_PHONETIC_PATTERNS)),
            (Group::Ongko, Patterns::new(ONGKO_PHONETIC_PATTERNS)),
            (Group::Diacritic, Patterns::new(DIACRITIC_PHONETIC_PATTERNS)),
            (Group::Biram, Patterns::new(BIRAM_PHONETIC_PATTERNS)),
            (Group::Prithayok, Patterns::new(PRITHAYOK_PHONETIC_PATTERNS)),
            (Group::Ae, Patterns::new(AE_PHONETIC_PATTERNS)),
        ]
        .into_iter()
        .collect();
        KhiproPhonetic { groups }
    }

    pub fn convert(&self, raw_input: &str) -> String {
        let mut output = String::with_capacity(64);
        self.convert_into(raw_input, &mut output);
        output
    }

    pub fn convert_into(&self, raw_input: &str, output: &mut String) {
        let mut state = State::Init;
        let mut input = &raw_input[0..];

        output.clear();
        while !input.is_empty() {
            let mut match_len = 0;
            let mut replacement = "";
            let mut join_char = false;
            let mut next_state = State::Init;

            for group in state.groups().iter() {
                if let Some(patterns) = self.groups.get(group) {
                    if let Some(pattern) = patterns.find_pattern(input) {
                        if pattern.find.len() > match_len {
                            join_char = state == State::Byanjon && *group == Group::Phola;
                            match_len = pattern.find.len();
                            replacement = pattern.default_replacement;
                            next_state = state.next_state(group);
                        }
                    }
                }
            }

            if match_len > 0 {
                if join_char {
                    output.push('্');
                }
                output.push_str(replacement);
                input = &input[match_len..];
                state = next_state;
            } else {
                input.chars().next().map(|c| output.push(c));
                input = &input[1..];
                state = State::Init;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // স্বরবর্ণ ও স্বরচিহ্ন
    #[test]
    fn test_khipro_words1() {
        let p = KhiproPhonetic::new();
        assert_eq!(p.convert("omor"), "অমর");
        assert_eq!(p.convert("amar"), "আমার");
        assert_eq!(p.convert("kaka"), "কাকা");
        assert_eq!(p.convert("iti"), "ইতি");
        assert_eq!(p.convert("ki"), "কি");
        assert_eq!(p.convert("iid"), "ঈদ");
        assert_eq!(p.convert("kii"), "কী");
        assert_eq!(p.convert("ui"), "উই");
        assert_eq!(p.convert("oju"), "অজু");
        assert_eq!(p.convert("uuru"), "ঊরু");
        assert_eq!(p.convert("kuup"), "কূপ");
        assert_eq!(p.convert("qju"), "ঋজু");
        assert_eq!(p.convert("kqtii"), "কৃতী");
        assert_eq!(p.convert("ebar"), "এবার");
        assert_eq!(p.convert("ke"), "কে");
        assert_eq!(p.convert("oik"), "ঐক");
        assert_eq!(p.convert("koi"), "কৈ");
        assert_eq!(p.convert("wstad"), "ওস্তাদ");
        assert_eq!(p.convert("kwn"), "কোন");
        assert_eq!(p.convert("oucitz"), "ঔচিত্য");
        assert_eq!(p.convert("nou"), "নৌ");
        assert_eq!(p.convert("aep"), "অ্যাপ");
        assert_eq!(p.convert("maep"), "ম্যাপ");
        assert_eq!(p.convert("hwatfsaefp"), "হোয়াটসঅ্যাপ");
        assert_eq!(p.convert("watfar"), "ওয়াটার");
        assert_eq!(p.convert("dhwa"), "ধোয়া");
        assert_eq!(p.convert("wedar"), "ওয়েদার");
        assert_eq!(p.convert("swetfar"), "সোয়েটার");
        assert_eq!(p.convert("harrdfwaer"), "হার্ডওয়্যার");
    }

    // স্বরবর্ণ ও স্বরচিহ্ন টিপ্‌স
    #[test]
    fn test_khipro_words2() {
        let p = KhiproPhonetic::new();
        assert_eq!(p.convert("kol"), "কল");
        assert_eq!(p.convert("oto"), "অত");
        assert_eq!(p.convert("am;ar"), "আমআর");
        assert_eq!(p.convert("chok;ka"), "ছককা");
        assert_eq!(p.convert("far"), "ার");
        assert_eq!(p.convert("boif"), "বই");
        assert_eq!(p.convert("bouf"), "বউ");
        assert_eq!(p.convert("bif"), "বই");
        assert_eq!(p.convert("ruup"), "রূপ");
        assert_eq!(p.convert("ruuffp"), "র‌ূপ");
    }

    // ব্যঞ্জনবর্ণ
    #[test]
    fn test_khipro_words3() {
        let p = KhiproPhonetic::new();
        assert_eq!(p.convert("kolom"), "কলম");
        assert_eq!(p.convert("k;l;m"), "কলম");
        assert_eq!(p.convert("khata"), "খাতা");
        assert_eq!(p.convert("garfi"), "গাড়ি");
        assert_eq!(p.convert("ghor"), "ঘর");
        assert_eq!(p.convert("gh;r"), "ঘর");
        assert_eq!(p.convert("rongo"), "রঙ");
        assert_eq!(p.convert("ranga"), "রাঙা");
        assert_eq!(p.convert("rangga"), "রাঙ্গা");
        assert_eq!(p.convert("ongk"), "অঙ্ক");
        assert_eq!(p.convert("ongfk"), "অংক");
        assert_eq!(p.convert("ca"), "চা");
        assert_eq!(p.convert("chata"), "ছাতা");
        assert_eq!(p.convert("jadughor"), "জাদুঘর");
        assert_eq!(p.convert("jhorf"), "ঝড়");
        assert_eq!(p.convert("minffa"), "মিঞা");
        assert_eq!(p.convert("jhonjha"), "ঝঞ্ঝা");
        assert_eq!(p.convert("tfaka"), "টাকা");
        assert_eq!(p.convert("ottf"), "অট্ট");
        assert_eq!(p.convert("tffandfa"), "ঠান্ডা");
        assert_eq!(p.convert("tfhandfa"), "ঠান্ডা");
        assert_eq!(p.convert("odfitf"), "অডিট");
        assert_eq!(p.convert("addfa"), "আড্ডা");
        assert_eq!(p.convert("dffaka"), "ঢাকা");
        assert_eq!(p.convert("dfhaka"), "ঢাকা");
        assert_eq!(p.convert("boronf"), "বরণ");
        assert_eq!(p.convert("tumi"), "তুমি");
        assert_eq!(p.convert("twmar"), "তোমার");
        assert_eq!(p.convert("thaka"), "থাকা");
        assert_eq!(p.convert("d;i"), "দই");
        assert_eq!(p.convert("dif"), "দই");
        assert_eq!(p.convert("dhakka"), "ধাক্কা");
        assert_eq!(p.convert("dhbni"), "ধ্বনি");
        assert_eq!(p.convert("nombor"), "নম্বর");
        assert_eq!(p.convert("n;mb;r"), "নম্বর");
        assert_eq!(p.convert("pakhi"), "পাখি");
        assert_eq!(p.convert("phol"), "ফল");
        assert_eq!(p.convert("barfi"), "বাড়ি");
        assert_eq!(p.convert("vorpur"), "ভরপুর");
        assert_eq!(p.convert("matfi"), "মাটি");
        assert_eq!(p.convert("zokhon"), "যখন");
        assert_eq!(p.convert("zkhn"), "যখন");
        assert_eq!(p.convert("rong"), "রং");
        assert_eq!(p.convert("lav"), "লাভ");
        assert_eq!(p.convert("shobd"), "শব্দ");
        assert_eq!(p.convert("susfom"), "সুষম");
        assert_eq!(p.convert("sob"), "সব");
        assert_eq!(p.convert("hawa"), "হাওয়া");
        assert_eq!(p.convert("upay"), "উপায়");
        assert_eq!(p.convert("jwrfa"), "জোড়া");
        assert_eq!(p.convert("asfarff"), "আষাঢ়");
    }

    // বিশেষ কিছু যুক্তবর্ণ এবং বিবিধ
    #[test]
    fn test_khipro_words4() {
        let p = KhiproPhonetic::new();
        assert_eq!(p.convert("gorrb"), "গর্ব");
        assert_eq!(p.convert("porrzay"), "পর্যায়");
        assert_eq!(p.convert("rzam"), "র‍্যাম");
        assert_eq!(p.convert("raem"), "র‍্যাম");
        assert_eq!(p.convert("shikfa"), "শিক্ষা");
        assert_eq!(p.convert("diikfa"), "দীক্ষা");
        assert_eq!(p.convert("oncol"), "অঞ্চল");
        assert_eq!(p.convert("poncash"), "পঞ্চাশ");
        assert_eq!(p.convert("bancha"), "বাঞ্ছা");
        assert_eq!(p.convert("gunjon"), "গুঞ্জন");
        assert_eq!(p.convert("bznjon"), "ব্যঞ্জন");
        assert_eq!(p.convert("jhonjhatf"), "ঝঞ্ঝাট");
        assert_eq!(p.convert("ggan"), "জ্ঞান");
        assert_eq!(p.convert("biggan"), "বিজ্ঞান");
        assert_eq!(p.convert("duggga"), "দুগ্গা");
        assert_eq!(p.convert("ottf"), "অট্ট");
        assert_eq!(p.convert("cottfgram"), "চট্টগ্রাম");
        assert_eq!(p.convert("addfa"), "আড্ডা");
        assert_eq!(p.convert("shusfk"), "শুষ্ক");
        assert_eq!(p.convert("misftfi"), "মিষ্টি");
        assert_eq!(p.convert("kasftff"), "কাষ্ঠ");
        assert_eq!(p.convert("kasftfh"), "কাষ্ঠ");
        assert_eq!(p.convert("usfn"), "উষ্ণ");
        assert_eq!(p.convert("basfp"), "বাষ্প");
        assert_eq!(p.convert("nisfphol"), "নিষ্ফল");
        assert_eq!(p.convert("kilqqn"), "কিল্ন");
        assert_eq!(p.convert("udxxdiin"), "উদ্‌দীন");
    }

    #[test]
    fn test_khipro_sentences() {
        let p = KhiproPhonetic::new();
        assert_eq!("ঘটোৎকচ", p.convert("ghotfwt/koc"));
        assert_eq!("আমার সোনার বাংলা", p.convert("amar swnar bangla"));
        assert_eq!("আমি বাংলায় গান গাই", p.convert("ami banglay gan gai"));
        assert_eq!(
            "আমাদের ভালোবাসা হয়ে গেল ঘাস, খেয়ে গেল গরু আর দিয়ে গেল বাঁশ",
            p.convert("amader valwbasa hoye gel ghas, kheye gel goru ar diye gelo ba/sh")
        );
    }
}
