# Zangar

<div align="center">
    <img src="https://github.com/ui-ce/zangar/raw/asli/zangar-small.png">
</div>

**Zangar** (Persian for _Rust_) allows you to write Rust programs in Persian, using Persian keywords and function names.

It is adapted from Rouille, the original French implementation of Rust.

### Example

```rust
zangar::zangar! {
    خارجی جعبه زنگار ;

    تابع اصلی () {
        باشد x = 31 ;

        تطبیق x {        
            42 =>  {
                چاپ!("چهل و دو")                
                        }
            _ => چاپ!("راست به زبان فارسی")            
        }
    }
}
```


```rust
zangar::zangar! {
    khareji jabe zangar;

    tb asli() {
        bashad x = 31;

        tatbigh x {        
            42 =>  {
                chap!("چهل و دو")                
            }
            _ => chap!("راست به زبان فارسی")            
        }
    }
}
```

See the [keywords](./zangar_proc_macro/src/lib.rs) and the [examples](./examples/src/main.rs) to get a rough sense of the whole syntax.

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Croatian: [hrđa](https://github.com/njelich/hrdja)
- Persian : [zangar (زنگار)](https://github.com/ui-ce/zangar)
- All of the above: [unirust](https://github.com/charyan/unirust)
