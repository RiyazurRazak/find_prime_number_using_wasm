use crate::utils::find_prime;
use crate::utils::generate_random_number;
use gloo::console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let number = use_state(|| 0);
    let max_range = use_state(|| 100000);
    let is_prime = use_state(|| false);
    //useeffect hook
    {
        let number = number.clone();
        let max_range = max_range.clone();
        let is_prime = is_prime.clone();
        use_effect_with_deps(
            move |_| {
                log!("UseEffect run");
                let random_number = generate_random_number::generate(*max_range);
                number.set(random_number);
                log!(random_number);
                is_prime.set(find_prime::isprime(random_number));

                // cleanup fn
                || ()
            },
            //dependency
            (),
        );
    }
    let generate_new_number = {
        let number = number.clone();
        let max_range = max_range.clone();
        let is_prime = is_prime.clone();
        Callback::from(move |_| {
            let random_number = generate_random_number::generate(*max_range);
            number.set(random_number);
            is_prime.set(find_prime::isprime(random_number));
        })
    };

    let find_is_it_prime = {
        let number = number.clone();
        let is_prime = is_prime.clone();
        Callback::from(move |_| {
            log!(find_prime::isprime(*number));
            is_prime.set(find_prime::isprime(*number));
        })
    };

    let range_change = {
        let number = number.clone();
        let max_range = max_range.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value: String = input.value();
            let n = value.parse::<u32>().unwrap();
            max_range.set(n);
            number.set(generate_random_number::generate(*max_range));
            input.set_value(&value);
        })
    };

    html! {
        <>
          <h1 class="title">{"Find if a number is prime or not"}</h1>
          <h3>{"The Generated Number is: "}{ *number }</h3>
          <div class="btn-container">
            <button class={classes!("btn", "btn-secondary")} onclick={find_is_it_prime} >{"Find"}</button>
          </div>
          <div class="btn-container">
          <button class={classes!("btn", "btn-success",)} onclick={generate_new_number}>{"Generate New Number"}</button>
          </div>
          <br/>
          <div>
            <h4>{"Adjust the random range from 1 to "}{*max_range}</h4>
            <input type="range" id="range" class="form-range" min={"1"} max={"100000"}  oninput={range_change} />
          <div class="range-values">
            <p>{"min : 1"}</p>
            <p>{"max : 100000"}</p>
          </div>
          <hr />
          <h2>{"The given "} {*number} {" is "}{ if *is_prime {"prime"} else {"not a prime"} }</h2>
        </div>

        <footer class="footer">
          <p>{"Developed by"}</p>
          <pre>{" "}</pre>
          <a href="https://riyazurrazak.com/">{"Riyazur Razak"}</a>
          <pre>{" "}</pre>
          <p>{"using web assembly"}</p>
        </footer>
        </>
    }
}
