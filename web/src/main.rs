/*use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (
        log(&format_args!( $( $t )* ).to_string())
    )
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}
*/
const INDEX: &str = include_str!("../templates/lazyfoo.html");

//#[wasm_bindgen(start)]
pub fn main()  {
    let mut tera = tera::Tera::default();
    let mut context = tera::Context::new();

      let lessons = [
        01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 32, 33, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
        51, 52,
    ]
    .into_iter()
    .enumerate()
    .collect::<std::collections::HashMap<usize, i32>>();

    context.insert("lesson", &"lesson02");
    context.insert("lessons", &lessons);

    // window.document.body.innerText=""
    let string = tera.render_str(INDEX, &context).unwrap();
    /*for (pos, lesson) in [
        01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 32, 33, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
        51, 52,
    ]
    .into_iter()
    .enumerate()
    {

    }*/


    //console_log!("{string}");
    println!("{string}");

   // Ok(())
}

// cargo build --target wasm32-unknown-unknown
// wasm-bindgen target/wasm32-unknown-unknown/debug/web.wasm --out-dir public/wasm/rust-lazy-foo --target web --no-typescript
/*
Ciao Tesoro, come va?
como amaneces?

Es un poco complicado hablar en la hora correcta contigo teniendo en cuenta que tenemos 6 horas de diferencia en nuestras zonas horario

Realmente No tiene importancia, hagamos lo que se pueda, si deseo conocerte, linda : )

Bien, antes de que esto continúe, ¡confirma que tienes un buen volumen de senos!, ¿cual es tu talla?
LOL. No es verdad, o quizá si : ), hahahaha, ahora enserio: Te cuento que desde joven me agrada la tecnología y lo moderno, esa forma de pensar me llevo a programar desde los trece años de edad por otro lado me impulso a desear vivir en Caracas. Hoy en día... joder, lo inesperado siempre aparece, en mi país nada es fácil menos moderno, tengo dos opciones para darme la vida que sueño:
1) Que cambie todo el país. - Dudo que suceda.
2) Migrar. - Lo decidí hace mucho.

No solo estudio lo correspondiente a mi carrera, también varias otras materias (ejemplo italiano, comercio, contabilidad, etc). XD cuando inicie a escuchar música en italiano mi madre se incomodaba, bueno aun es así.


*/