use lemmeknow::Identify;
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
    let id = Identify::default();
    let r = id.identify(&vec![props.text.clone()]);

    if r.len() == 0 {
        return html! {
            <div class="err">
            {"No Possible Identifications found :("}
            </div>
        };
    };

    let tdara = r
        .iter()
        .map(|f| {
            let description = match (&f.data.Description, &f.data.URL) {
                (Some(des), Some(url)) => html! {<span>{des}<br /><a href={format!("{url}{}", &f.text)}>{"URL"}</a></span>},
                (Some(des), None) => html!{des},
                (None, Some(url)) => html!{<a href={format!("{url}{}", &f.text)}>{"URL"}</a>},
                (None, None) => html!{<span>{"None"}</span>},
            };
            let tags = f.data.Tags.join(", ");
            html! {
            <tr>
                <td>{f.data.Name.clone()}</td>
                <td>{description}</td>
                <td>{tags}</td>
                <td>{f.data.Rarity}</td>
            </tr>
            }
        })
        .collect::<Html>();

    html! {
        <div class="output">
            <a class="download_button" download="res.json" href={format!("data:text/json;charset=utf-8,{}", Identify::to_json(&r))}>
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
        <h1 class="title">{"lemmeknow"}</h1>
        <div class="icons">
        <a href="https://github.com/swanandx/lemmeknow"><i class="bx bxl-github" style="color:#ffffff"  ></i></a>
        <a href="https://twitter.com/_swanandx"><i class="bx bxl-twitter" style="color:#ffffff"  ></i></a>
        </div>
        <h4 class="subtitle">{"⚡🦀 fastest way to identify anything 🔍⚡"}</h4>
        <form class="main__form" {onsubmit}>
        <input class="form__input_text" type="text" ref={input_ref} placeholder="Enter text to identify" />
        <button class="form__button">{"Identify"}</button>
        </form>
        <Identifications text={(*text_state).clone()} />
        </div>
        <footer><span>{"Made with "}<i style="color:#ff0000" class="bx bxs-heart bx-tada"></i>{" by "}<a href="https://swanandx.github.io">{"swanandx"}</a></span></footer>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
