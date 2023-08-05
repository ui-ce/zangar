zangar::zangar! {
    khareji jabe zangar;

    est std::collections::Negasht ba Neg;

    rabet KelidMeghdar {
        tb benevis(&ght khod, kelid: Reshte, meghdar: Reshte);
        tb bekhan(&khod, kelid: Reshte) -> Natije<Entekhab<&Reshte>, Reshte>;
    }


    sakhteman Diksheneri {
        negasht: Entekhab<Neg<Reshte, Reshte>> ,
    }

    ejra Diksheneri {
        tb new() -> Self {
            bashad res = Self { negasht : Hich, };
            res
        }
        tb chap_kon(&khod) {
            agar bashad Ghadri(ng) = &khod.negasht {
                baraye (k, m) dar ng {
                    chap!("{k} : {m}");
                }
            }
        }
    }

    ejra KelidMeghdar baraye Diksheneri {
        tb benevis(&ght khod, kelid: Reshte, meghdar: Reshte) {
            bashad dik = khod.negasht.begir_ya_darj_kon(Pishfarz::pishfarz);
            dik.darj(kelid, meghdar);
        }
        tb bekhan(&khod, kelid: Reshte) -> Natije<Entekhab<&Reshte>, Reshte> {
            agar bashad Ghadri(dik) = khod.negasht.ba_marja() {
                Ok(dik.begir(&kelid))
            } garna {
                Kh("khandan diksheneri".be())
            }
        }
    }

    nahamzaman tb mesal() {
    }

    nahamzaman tb mesal2() {
        mesal().entezar;
    }

    aam(jabe) tb shayad(i: u32) -> Entekhab<Natije<u32, Reshte>> {
        agar i % 2 == 1 {
            agar i == 42 {
                Ghadri(Kh(Reshte::az("eshtebah")))
            } garna {
                Ghadri(Ok(33))
            }
        } garna {
            Hich
        }
    }

    tb asli () {

        bashad ght dik : Diksheneri = Diksheneri::new();
        dik.benevis("0".to_string(), "صفر = sefr = zero".to_string());
        dik.benevis("1".to_string(), "یک = yek = one".to_string());
        dik.benevis("2".to_string(), "دو = do = two".to_string());
        dik.benevis("3".to_string(), "سه = se = three".to_string());
        dik.benevis("4".to_string(), "چهار = chahar = four".to_string());
        dik.benevis("5".to_string(), "پنج = panj = five".to_string());
        dik.benevis("6".to_string(), "شش = shesh = six".to_string());
        dik.benevis("7".to_string(), "هفت = haft = seven".to_string());
        dik.benevis("8".to_string(), "هشت = hasht = eight".to_string());
        dik.benevis("9".to_string(), "نه = noh = nine".to_string());
        dik.benevis("10".to_string(), "ده = dah = ten".to_string());
       
        dik.chap_kon();

        bashad ght x = 31 ;

        tatbigh x {
            42 => {
                chap!("چهل و دو")
            }
            _ => chap!("راست به زبان فارسی")
        }

        baraye i dar 0..10 {
            bashad val = halghe {
                tavaghof i;
            };

            tavaghti x < val {
                x += 1;
            }

            x = agar bashad Ghadri(natije) = shayad(i) {
                natije.vapich()
            } garna {
                12
            };
        }

        //dovvom();
    }

    #[ejaze(dastnayaftani)]
    tb dovvom() {
        ekhtar!("naa!"); // for the true Persian experience
    }
}
