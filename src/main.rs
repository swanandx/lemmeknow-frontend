use lemmeknow::Identifier;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ToIdentify {
    text: String,
}

#[function_component(Identifications)]
fn identify_text_as(props: &ToIdentify) -> Html {
    if props.text.is_empty() {
        return html! {};
    }
    let id = Identifier::default();
    let r = id.identify(&[props.text.clone()]);

    if r.is_empty() {
        return html! {
            <div class="err">
            {"No Possible Identifications found :("}
            </div>
        };
    };

    let tdara = r
        .iter()
        .map(|f| {
            let description = match (&f.data.description, &f.data.url) {
                (Some(des), Some(url)) => html! {<span>{des}<br /><a href={format!("{url}{}", &f.text)}>{"URL"}</a></span>},
                (Some(des), None) => html!{des},
                (None, Some(url)) => html!{<a href={format!("{url}{}", &f.text)}>{"URL"}</a>},
                (None, None) => html!{<span>{"None"}</span>},
            };
            let tags = f.data.tags.join(", ");
            html! {
            <tr>
                <td>{f.data.name.clone()}</td>
                <td>{description}</td>
                <td>{tags}</td>
                <td>{f.data.rarity}</td>
            </tr>
            }
        })
        .collect::<Html>();

    let jar = js_sys::Array::from(&Identifier::to_json(&r).into());

    let js_value = wasm_bindgen::JsValue::from(jar);

    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &js_value,
        web_sys::BlobPropertyBag::new().type_("text/json"),
    )
    .unwrap();
    let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    html! {
        <div class="output">
            <a class="download_button" download="results.json" href={download_url}>
            {"Download as JSON"}
        </a>
        <div class="tbl-header">

    <table cellpadding="0" cellspacing="0" border="0">
      <thead>
        <tr>
          <th>{"Match"}</th>
          <th>{"Description"}</th>
          <th>{"Tags"}</th>
          <th>{"Rarity"}</th>
        </tr>
      </thead>
    </table>
    </div>
    <div class="tbl-content">
      <table cellpadding="0" cellspacing="0" border="0">
        <tbody>
        {tdara}
        </tbody>
      </table>
    </div>
    </div>
      }
}

#[function_component(App)]
fn app() -> Html {
    let text_state = use_state(|| "".to_string());
    let input_ref = use_node_ref();

    let onsubmit = {
        let input_ref = input_ref.clone();
        let text_state = text_state.clone();

        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                text_state.set(input.value());
            }
        })
    };

    html! {
        <>
        <div class="main">
        <div class="intro">
        <h1 class="title">{"lemmeknow"}</h1>
        <iframe src="https://github.com/sponsors/swanandx/button" title="Sponsor swanandx" height="35" width="116" style="border: 0;"></iframe>
        </div>
        <div class="icons">
        <a href="https://github.com/swanandx/lemmeknow"><i class="bx bxl-github" style="color:#ffffff"  ></i></a>
        <a href="https://twitter.com/_swanandx"><i class="bx bxl-twitter" style="color:#ffffff"  ></i></a>
        </div>
        <h4 class="subtitle">{"⚡🦀 fastest way to identify anything 🔍⚡"}</h4>
        <form class="main__form" {onsubmit}>
        <input class="form__input_text" type="text" ref={input_ref} placeholder="Enter text to identify e.g. UC11L3JDgDQMyH8iolKkVZ4w" />
        <button class="form__button">{"Identify"}</button>
        </form>
        <Identifications text={(*text_state).clone()} />
        </div>
        <div class="coffee">
        <h4>{"Support lemmeknow <3"}</h4>
        <a href="https://www.buymeacoffee.com/swanandx" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/arial-yellow.png" alt="Buy Me A Coffee" style="width: 175px !important;" /></a>
        </div>
        <footer><span>{"Made with "}<i style="color:#ff0000" class="bx bxs-heart bx-tada"></i>{" by "}<a href="https://swanandx.github.io">{"swanandx"}</a></span></footer>
        
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
