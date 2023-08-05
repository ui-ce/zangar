use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "خبط" => "Err",
        "خوب" => "Ok",
        "رشته" => "String",
        "نگاشت" => "HashMap",
        "پیش‌فرض" => "Default",
        "خطا" => "Error",
        "انتخاب" => "Option",
        "قدری" => "Some",
        "هیچ" => "None",
        "نتیجه" => "Result",
        "خویش" => "Self",
        "چاپ" => "println",
        "توقف" => "break",
        "ناهمزمان" => "async",
        "انتظار" => "await",
        "حلقه" => "loop",
        "نقل" => "move",
        "جعبه" => "crate",
        "دست‌نیافتنی" => "unreachable_code",
        "بعنوان" => "as",
        "ثابت" => "const",
        "رابط" => "trait",
        "ناامن" => "unsafe",
        "در" => "in",
        "از" => "from",
        "پویا" => "dyn",
        "واپیچ" => "unwrap",
        "فرض" => "default",
        "بمرجع" => "as_ref",
        "وخ" => "io",
        "خارجی" => "extern",
        "نادرست" => "false",
        "تابع" => "fn",
        "مافوق" => "super",
        "درج" => "insert",
        "بگیر" => "get",
        "اجازه" => "allow",
        "اخطار" => "panic",
        "مد" => "mod",
        "قت" => "mut",
        "جدید" => "new",
        "جاییکه" => "where",
        "برای" => "for",
        "بگیریادرج‌کن" => "get_or_insert_with",
        "اصلی" => "main",
        "عام" => "pub",
        "هیچ؟" => None?,
        "بازگشت" => "return",
        "اجرا" => "impl",
        "مرجع" => "ref",
        "تطبیق" => "match",
        "اگر" => "if",
        "وگرنه" => "else",
        "خود" => "self",
        "باشد" => "let",
        "ایستا" => "static",
        "ساختمان" => "struct",
        "توقع" => "expect",
        "تاوقتی" => "while",
        "استفاده" => "use",
        "به" => "into",
        "درست" => "true",
        "شمارشی" => "enum",
        "گروه" => "Group",
        "شناسه" => "Ident",
        "جریان‌توکن" => "TokenStream",
        "درخت‌توکن" => "TokenTree",
        "برشته" => "to_string",
        "چون‌رشته" => "as_str",
        "پوشش" => "span",
        "وکتور" => "Vec",
        "جریان" => "stream",
        "نشاندن" => "push",
        "توسعه" => "extend",
        "حایل" => "delimiter",
        "نشانه" => "Punct",
        "لفظی" => "Literal",
        "ماکرورویه" => "proc_macro",
        "زنگار" => "zangar",
        "Kh" => "Err",
        "Ok" => "Ok",
        "Reshte" => "String",
        "Negasht" => "HashMap",
        "Pishfarz" => "Default",
        "Khata" => "Error",
        "Entekhab" => "Option",
        "Ghadri" => "Some",
        "Hich" => "None",
        "Natije" => "Result",
        "Khod" => "Self",
        "chap" => "println",
        "tavaghof" => "break",
        "nahamzaman" => "async",
        "entezar" => "await",
        "halghe" => "loop",
        "naghl" => "move",
        "jabe" => "crate",
        "dastnayaftani" => "unreachable_code",
        "ba" => "as",
        "sabet" => "const",
        "rabet" => "trait",
        "naamn" => "unsafe",
        "dar" => "in",
        "az" => "from",
        "pouya" => "dyn",
        "pya" => "dyn",
        "vapich" => "unwrap",
        "pishfarz" => "default",
        "ba_marja" => "as_ref",
        "vk" => "io",
        "khareji" => "extern",
        "nadorost" => "false",
        "tb" => "fn",
        "mafogh" => "super",
        "darj" => "insert",
        "begir" => "get",
        "ejaze" => "allow",
        "ekhtar" => "panic",
        "md" => "mod",
        "ght" => "mut",
        "jadid" => "new",
        "jayike" => "where",
        "baraye" => "for",
        "begir_ya_darj_kon" => "get_or_insert_with",
        "asli" => "main",
        "aam" => "pub",
        "Hich?" => None?,
        "bazgasht" => "return",
        "ejra" => "impl",
        "marja" => "ref",
        "tatbigh" => "match",
        "agar" => "if",
        "garna" => "else",
        "khod" => "self",
        "bashad" => "let",
        "ista" => "static",
        "sakhteman" => "struct",
        "tavagho" => "expect",
        "tavaghti" => "while",
        "est" => "use",
        "be" => "into",
        "dorost" => "true",
        "shomareshi" => "enum",
        "Gorouh" => "Group",
        "Shenase" => "Ident",
        "JaryanToken" => "TokenStream",
        "DerakhtToken" => "TokenTree",
        "be_reshte" => "to_string",
        "ba_reshte" => "as_str",
        "poushesh" => "span",
        "Vec" => "Vec",
        "jaryan" => "stream",
        "vared" => "push",
        "tosee" => "extend",
        "hayel" => "delimiter",
        "Neshane" => "Punct",
        "Lafzi" => "Literal",
        "macro_ravie" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn zangar(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
