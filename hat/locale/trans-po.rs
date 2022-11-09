use std::*;
use std::collections::HashMap;

const __author__: _ = "Rory McCann <rory@technomancy.org>";
const __version__: _ = "1.0";
const __licence__: _ = "GPLv3";
let try_dummy = { //unsupported
};
let except!() = { //unsupported
println ! ("{:?} {:?} ", "failed to translate", sys.argv[1]);
exit(0);
};
fn translate_subpart<T0, T1, RT>(string: T0, lang: T1) -> RT {
    "Simple translate for just a certin string";
    println!("{:?} {:?} {:?} ", "translate", string, lang);
    let translater = subprocess.Popen(vec!["trans", "-b", (":" + lang)], subprocess.PIPE, subprocess.PIPE);
    translater.stdin.write((string + "
").encode());
    let (stringout, _) = translater.communicate();
    let stringout = stringout[..-1].decode("utf8");
    if stringout {
        println!("{:?} {:?} {:?} {:?} ", "translated", string, lang, stringout);
        return stringout;
    }
    time.sleep(1);
    println!("{:?} {:?} ", "failed to translate ", string);
    return "";
}

fn translate<T0, T1, RT>(string: T0, lang_direction: T1) -> RT {
    "Takes a string that is to be translated and returns the translated string, doesn't translate the %(format)s parts, they must remain the same text as the msgid";
    let named_format_regex = re.compile("%\([^\)]+?\)[sd]", re.VERBOSE);
    let matches = named_format_regex.findall(string);
    let mut new = None;
    if matches.len() == 0 {
        assert!(string.iter().all(|&x| x != "%("));
        new = translate_subpart(string, lang_direction);
    } else {
        let mut full_trans = translate_subpart(string, lang_direction);
        for match_ in matches {
            let mut translated_match = translate_subpart(match_, lang_direction);
            let translated_match_match = named_format_regex.search(translated_match);
            assert!(translated_match_match);
            translated_match = translated_match_match.group(0);
            let replace = re.compile(re.escape(translated_match), re.IGNORECASE);
            full_trans = replace.sub(match_, full_trans);
        }
        new = full_trans;
    }
    return new;
}

fn translate_po<T0, T1>(filename: T0, lang: T1) {
    "Given a .po file, Translate it";
    let pofile = polib.pofile(filename);
    pofile.metadata["Plural-Forms"] = "nplurals=2; plural=(n != 1)";
    let mut save = false;
    let try_dummy = { //unsupported
        for entry in pofile {
            if entry.msgstr {
                continue;
            }
            if entry.msgid_plural == "" {
                entry.msgstr = translate(entry.msgid, lang);
                if !entry.msgstr {
                    break;
                }
                save = true;
            } else {
                entry.msgstr_plural["0"] = translate(entry.msgid, lang);
                entry.msgstr_plural["1"] = translate(entry.msgid_plural, lang);
            }
        }
    };
}

fn main() {
    translate_po(sys.argv[1], sys.argv[2]);
}