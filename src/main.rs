use yew::{html, function_component, Callback, InputEvent, Properties};

fn main() {
    yew::start_app::<App>();

}
#[function_component(App)]
fn app() -> Html {
    
    
    html!{
        <main>
            <VideoControls/>
            <VideoSection name="noomber video" id="tmYhb0efRIw"/>
        
        </main>
    }
}

#[function_component(VideoControls)]
fn controls() -> Html {
    let handle_input: Callback<InputEvent>= Callback::from(|_| {});
    html!{
    <div>
        <div>
            {"ingresa una palabra"}
        </div>
        <div>
            <input type="text" oninput={handle_input}/>
        </div>
        <div><button>{"buscar!"}</button></div>
    </div>
    }
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps {
    id: String,
    name: String,
}

#[function_component(VideoSection)]
fn video_section(props: &VideoSectionProps) -> Html {
    
    let yt_url = format!("https://www.youtube.com/embed/{}", props.id);
    html!{
        <div>
            <iframe width="560" height="315" src={yt_url}></iframe>
        </div>
    }
}
