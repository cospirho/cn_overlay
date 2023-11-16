// converts the cedict_ts.u8 file to a tsv file
// with character, pinyin, definition
// just makes it easier to parse and add new words
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use std::collections::BTreeMap;

// mostly from https://github.com/quizlet/pinyin-converter/blob/master/src/pinyin_converter.coffee
// it can definitely be improved, but doesn't really matter

struct PinyinConverter {
    pinyin_regex: Regex,
    vowels: Vec<&'static str>,
    pinyin: Vec<Vec<&'static str>>,
}

impl PinyinConverter {
    fn new() -> Self {
        let pinyin_regex = Regex::new(r"(?i)(shuang|chuang|zhuang|xiang|qiong|shuai|niang|guang|sheng|kuang|shang|jiong|huang|jiang|shuan|xiong|zhang|zheng|zhong|zhuai|zhuan|qiang|chang|liang|chuan|cheng|chong|chuai|hang|peng|chuo|piao|pian|chua|ping|yang|pang|chui|chun|chen|chan|chou|chao|chai|zhun|mang|meng|weng|shai|shei|miao|zhui|mian|yong|ming|wang|zhuo|zhua|shao|yuan|bing|zhen|fang|feng|zhan|zhou|zhao|zhei|zhai|rang|suan|reng|song|seng|dang|deng|dong|xuan|sang|rong|duan|cuan|cong|ceng|cang|diao|ruan|dian|ding|shou|xing|zuan|jiao|zong|zeng|zang|jian|tang|teng|tong|bian|biao|shan|tuan|huan|xian|huai|tiao|tian|hong|xiao|heng|ying|jing|shen|beng|kuan|kuai|nang|neng|nong|juan|kong|nuan|keng|kang|shua|niao|guan|nian|ting|shuo|guai|ning|quan|qiao|shui|gong|geng|gang|qian|bang|lang|leng|long|qing|ling|luan|shun|lian|liao|zhi|lia|liu|qin|lun|lin|luo|lan|lou|qiu|gai|gei|gao|gou|gan|gen|lao|lei|lai|que|gua|guo|nin|gui|niu|nie|gun|qie|qia|jun|kai|kei|kao|kou|kan|ken|qun|nun|nuo|xia|kua|kuo|nen|kui|nan|nou|kun|jue|nao|nei|hai|hei|hao|hou|han|hen|nai|rou|xiu|jin|hua|huo|tie|hui|tun|tui|hun|tuo|tan|jiu|zai|zei|zao|zou|zan|zen|eng|tou|tao|tei|tai|zuo|zui|xin|zun|jie|jia|run|diu|cai|cao|cou|can|cen|die|dia|xue|rui|cuo|cui|dun|cun|cin|ruo|rua|dui|sai|sao|sou|san|sen|duo|den|dan|dou|suo|sui|dao|sun|dei|zha|zhe|dai|xun|ang|ong|wai|fen|fan|fou|fei|zhu|wei|wan|min|miu|mie|wen|men|lie|chi|cha|che|man|mou|mao|mei|mai|yao|you|yan|chu|pin|pie|yin|pen|pan|pou|pao|shi|sha|she|pei|pai|yue|bin|bie|yun|nüe|lve|shu|ben|ban|bao|bei|bai|lüe|nve|ren|ran|rao|xie|re|ri|si|su|se|ru|sa|cu|ce|ca|ji|ci|zi|zu|ze|za|hu|he|ha|ju|ku|ke|qi|ka|gu|ge|ga|li|lu|le|qu|la|ni|xi|nu|ne|na|ti|tu|te|ta|xu|di|du|de|bo|lv|ba|ai|ei|ao|ou|an|en|er|da|wu|wa|wo|fu|fo|fa|nv|mi|mu|yi|ya|ye|me|mo|ma|pi|pu|po|yu|pa|bi|nü|bu|lü|e|o|a)r?[1-5]").expect("Failed to create pinyin regex");

        let vowels = vec!["a*","e*","i*","o*","u*","ü*","A*","E*","I*","O*","U*","Ü*"];

        let pinyin = vec![
            vec!["ā", "ē", "ī", "ō", "ū", "ǖ", "Ā", "Ē", "Ī", "Ō", "Ū", "Ǖ"],
            vec!["á", "é", "í", "ó", "ú", "ǘ", "Á", "É", "Í", "Ó", "Ú", "Ǘ"],
            vec!["ǎ", "ě", "ǐ", "ǒ", "ǔ", "ǚ", "Ǎ", "Ě", "Ǐ", "Ǒ", "Ǔ", "Ǚ"],
            vec!["à", "è", "ì", "ò", "ù", "ǜ", "À", "È", "Ì", "Ò", "Ù", "Ǜ"],
            vec!["a", "e", "i", "o", "u", "ü", "A", "E", "I", "O", "U", "Ü"],
        ];

        PinyinConverter {
            pinyin_regex,
            vowels,
            pinyin,
        }
    }

    fn convert(&self, string: &str) -> String {
        let mut result = String::from(string);

        while let Some(captures) = self.pinyin_regex.find(&result) {
            let matched_str = captures.as_str();
            let replacement = self.get_replacement(matched_str);
            result = result.replacen(matched_str, &replacement, 1);
        }

        result
    }

    fn get_replacement(&self, match_str: &str) -> String {
        let accent_map = self.get_accent_map();
        let tone: usize = match_str.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;
        let word = match_str.trim_end_matches(char::is_numeric).replace('v', "ü").replace('V', "Ü");
        for (base, vowel) in &accent_map {
            if word.contains(base) {
                let vowel_char = vowel.chars().next().unwrap();
                for (vowel_num, vowel_num_str) in self.vowels.iter().enumerate() {
                    if vowel_num_str.contains(vowel_char) {
                        let accented_vowel_char = self.pinyin[tone][vowel_num];
                        //the accented vowel is a different length
                        let starred_word = word.replace(base, vowel);
                        let mut replaced_word = String::new();
                        let mut found_vowel = false;
                        for c in starred_word.chars() {
                            if c == vowel_char {
                                replaced_word.push_str(accented_vowel_char);
                                found_vowel = true;
                            } else {
                                if found_vowel {
                                    found_vowel = false;
                                    continue
                                }
                                replaced_word.push(c);
                            }
                        }
                        return replaced_word;
                    }
                }
            }
        }
        match_str.to_string()
    }

    // BTreeMap is used because it's sorted
    fn get_accent_map(&self) -> BTreeMap<String, String> {
        let stars = "a*i a*o e*i ia* ia*o ie* io* iu* \
                     A*I A*O E*I IA* IA*O IE* IO* IU* \
                     o*u ua* ua*i ue* ui* uo* üe* \
                     O*U UA* UA*I UE* UI* UO* ÜE* \
                     A* E* I* O* U* Ü* \
                     a* e* i* o* u* ü*";
    
        let nostars = stars.replace("*", "");
        let nostars_list: Vec<_> = nostars.split_whitespace().map(|s| s.to_string()).collect();
        let stars_list: Vec<_> = stars.split_whitespace().map(|s| s.to_string()).collect();
        let mut accent_map = BTreeMap::new();
    
        for (nostar, star) in nostars_list.into_iter().zip(stars_list) {
            accent_map.insert(nostar, star);
        }
    
        accent_map
    }
}

fn sample_converter() {
    let converter = PinyinConverter::new();
    let result = converter.convert("qiao3");
    println!("{}", result);
    let result1 = converter.convert("san1ren2xing2bi4you3wo3shi1");
    println!("{}", result1);
    let result2 = converter.convert("lv4");
    println!("{}", result2);
    let result3 = converter.convert("WO3 HEN3 XI3HUAN QUIZLET!");
    println!("{}", result3);
}

fn main() {
    let input_file = File::open("cedict_ts.u8").expect("Failed to open input file");
    let reader = BufReader::new(input_file);

    let mut dictionary = String::new();
    let re = Regex::new(r"(.*) (.*) \[(.*)\] /(.*)/").expect("Failed to create regex");
    let pinyin_converter = PinyinConverter::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("#") {
                continue;
            }

            if let Some(captures) = re.captures(&line) {
                let entry = format!(
                    "{}\t{}\t{}\t\n",
                    captures.get(2).unwrap().as_str(),
                    pinyin_converter.convert(captures.get(3).unwrap().as_str()),
                    pinyin_converter.convert(captures.get(4).unwrap().as_str()),
                );
                dictionary.push_str(&entry);
            }
        }
    }

    let mut output_file = File::create("dictionary.tsv").expect("Failed to create output file");
    output_file.write_all(dictionary.as_bytes()).expect("Failed to write to output file");
}
