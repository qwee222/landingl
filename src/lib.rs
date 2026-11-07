use wasm_bindgen::prelude::*;
use js_sys::Math;

#[wasm_bindgen]
pub fn get_translation(lang: &str) -> js_sys::Object {
    let obj = js_sys::Object::new();

    let (title, q1, yes, no, q2, accept) = match lang {
        "es" => (
            "Discreto y fotos exclusivas",
            "¿Ofrezco servicios de citas, baile y masajes en casa, te gustaría ver mi catálogo?",
            "Sí", "No",
            "AVISO: ¿Estás de acuerdo en respetar a las personas que aparecen a continuación? Si conoces a alguien, guárdalo en privado...",
            "ACEPTAR"
        ),
        "ro" => (
            "Noapte discreta si fotografii exclusive",
            "Ofer servicii de intalniri, dans si masaj acasa, ai vrea sa vezi catalogul meu?",
            "Da", "Nu",
            "NOTA: Esti de acord sa respecti persoanele afisate mai jos? Daca cunosti pe cineva, pastreaza secretul...",
            "ACCEPT"
        ),
        "hu" => (
            "Diszkrét éjszaka, fényképek",
            "Szexuális szolgáltatásokat, táncot és erotikus masszázst kínálok otthon, szeretnéd látni a katalógusomat?",
            "Igen", "Nem",
            "MEGJEGYZÉS: Beleegyezel, hogy tiszteletben tartod az alább látható meztelen nöket? Ha valakit ismersz, ne mondd el senkinek...",
            "ELFOGADOM"
        ),
        "en" => (
            "Discreet and exclusive photos",
            "I offer home dates, dancing, and massage services, would you like to see my catalog?",
            "Yes", "No",
            "NOTICE: Do you agree to respect the people shown below? If you know anyone, keep it private...",
            "ACCEPT"
        ),
        "de" => (
            "Diskrete Nacht und exklusive Fotos",
            "Ich biete Hausbesuche, Tanz und Massagen an, möchtest du meinen Katalog sehen?",
            "Ja", "Nein",
            "HINWEIS: Stimmst du zu, die unten gezeigten Personen zu respektieren? Wenn du jemanden kennst, behalte es für dich...",
            "AKZEPTIEREN"
        ),
        "fr" => (
            "Photos discrètes et exclusives",
            "Je propose des rendez-vous a domicilio, de la danse et des massages, aimerais-tu voir mon catalogue ?",
            "Oui", "Non",
            "AVIS : Acceptes-tu de respecter les personnes présentées ci-dessous ? Si tu connais quelqu'un, garde-le pour toi...",
            "ACCEPTER"
        ),
        "it" => (
            "Foto discrete ed esclusive",
            "Offro appuntamenti a domicilio, ballo e massaggi, vorresti vedere il mio catalogo?",
            "Sì", "No",
            "AVVISO: Accetti di rispettare le persone mostrate di seguito? Se conosci qualcuno, tienilo per te...",
            "ACCETTA"
        ),
        "pt" => (
            "Fotos discretas e exclusivas",
            "Ofereço serviços de encontros, dança e massagem em casa, gostaria de ver meu catálogo?",
            "Sim", "Não",
            "AVISO: Você concorda em respeitar as pessoas mostradas abaixo? Se você conhece alguém, mantenha em sigilo...",
            "ACEITAR"
        ),
        "pl" => (
            "Dyskretne i ekskluzywne zdjecia",
            "Oferuje spotkania, taniec i uslugi masazu w domu, czy chcesz zobaczyc mój katalog?",
            "Tak", "Nie",
            "UWAGA: Czy zgadzasz sie szanowac osoby przedstawione ponizej? Jesli kogos znasz, zachowaj to dla siebie...",
            "AKCEPTUJ"
        ),
        "cs" => (
            "Diskrétní noc a exkluzivní fotky",
            "Nabízím domácí rande, tanec a masáže, chteli byste videt muj katalog?",
            "Ano", "Ne",
            "UPOZORNENÍ: Souhlasíte s respektováním osob zobrazených níže? Pokud nekoho znáte, nechte si to pro sebe...",
            "SOUHLASÍM"
        ),
        _ => (
            "Discreet and exclusive photos",
            "I offer home dates, dancing, and massage services, would you like to see my catalog?",
            "Yes", "No",
            "NOTICE: Do you agree to respect the people shown below? If you know anyone, keep it private...",
            "ACCEPT"
        ),
    };

    js_sys::Reflect::set(&obj, &JsValue::from_str("title"), &JsValue::from_str(title)).unwrap();
    js_sys::Reflect::set(&obj, &JsValue::from_str("q1"),    &JsValue::from_str(q1)).unwrap();
    js_sys::Reflect::set(&obj, &JsValue::from_str("yes"),   &JsValue::from_str(yes)).unwrap();
    js_sys::Reflect::set(&obj, &JsValue::from_str("no"),    &JsValue::from_str(no)).unwrap();
    js_sys::Reflect::set(&obj, &JsValue::from_str("q2"),    &JsValue::from_str(q2)).unwrap();
    js_sys::Reflect::set(&obj, &JsValue::from_str("accept"),&JsValue::from_str(accept)).unwrap();

    obj
}

#[wasm_bindgen]
pub fn generate_ip() -> String {
    let r = |max: f64| (Math::random() * max).floor() as u32;
    format!(
        "{}.{}.{}.{}",
        r(190.0) + 10,
        r(255.0),
        r(255.0),
        r(254.0) + 1
    )
}

#[derive(Clone)]
struct NotifItem {
    msg: String,
    flag: String,
    foto: String,
}

#[wasm_bindgen]
pub struct NotificationPool {
    items: Vec<NotifItem>,
    available: Vec<usize>,
}

#[wasm_bindgen]
impl NotificationPool {
    #[wasm_bindgen(constructor)]
    pub fn new() -> NotificationPool {
        let items = vec![
            NotifItem { msg: "Sikeres regisztráció".into(),      flag: "https://flagcdn.com/w40/hu.png".into(), foto: "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?w=100&h=100&fit=crop".into() },
            NotifItem { msg: "Úspešne zaregistrováno".into(),   flag: "https://flagcdn.com/w40/cz.png".into(), foto: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&h=100&fit=crop".into() },
            NotifItem { msg: "Erfolgreich registriert".into(),  flag: "https://flagcdn.com/w40/de.png".into(), foto: "https://images.unsplash.com/photo-1519085360753-af0119f7cbe7?w=100&h=100&fit=crop".into() },
            NotifItem { msg: "Inscrit avec succès".into(),      flag: "https://flagcdn.com/w40/fr.png".into(), foto: "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?w=100&h=100&fit=crop".into() },
            NotifItem { msg: "Registrado con éxito".into(),     flag: "https://flagcdn.com/w40/es.png".into(), foto: "https://images.unsplash.com/photo-1522075469751-3a6694fb2f61?w=100&h=100&fit=crop".into() },
            NotifItem { msg: "Registrato con successo".into(),  flag: "https://flagcdn.com/w40/it.png".into(), foto: "https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?w=100&h=100&fit=crop".into() },
            NotifItem { msg: "Zarejestrowano pomyslnie".into(), flag: "https://flagcdn.com/w40/pl.png".into(), foto: "https://images.unsplash.com/photo-1527980965255-d3b416303d12?w=100&h=100&fit=crop".into() },
            NotifItem { msg: "Înregistrat cu succes".into(),     flag: "https://flagcdn.com/w40/ro.png".into(), foto: "https://images.unsplash.com/photo-1570295999919-56ceb5ecca61?w=100&h=100&fit=crop".into() },
        ];
        let len = items.len();
        NotificationPool {
            items,
            available: (0..len).collect(),
        }
    }

    pub fn get_next(&mut self) -> js_sys::Object {
        if self.available.is_empty() {
            self.available = (0..self.items.len()).collect();
        }
        let idx = (Math::random() * self.available.len() as f64).floor() as usize % self.available.len();
        let real_idx = self.available.remove(idx);
        let it = &self.items[real_idx];

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &JsValue::from_str("msg"),  &JsValue::from_str(&it.msg)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("flag"), &JsValue::from_str(&it.flag)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("foto"), &JsValue::from_str(&it.foto)).unwrap();
        obj
    }
}
