use crate::prelude::*;
use leptos::*;

#[component]
pub fn Quicksearch(
    cx: Scope,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(cx, false);
    view! { cx,
        <leptonic-quicksearch>
            <leptonic-quicksearch-trigger on:click=move |_| set_show_modal.set(true)>
                "Search"
            </leptonic-quicksearch-trigger>
            <QuicksearchModal
                show_when=show_modal
                query=query
                on_cancel=move || set_show_modal.set(false)
            />
        </leptonic-quicksearch>
    }
}

#[derive(Debug, Clone)]
pub struct QuicksearchOption {
    pub view: Callback<Scope, View>,
    pub on_select: Callback<()>,
}

#[component]
fn QuicksearchModal<C>(
    cx: Scope,
    #[prop(into)] show_when: Signal<bool>,
    #[prop(into)] query: Callback<String, Vec<QuicksearchOption>>,
    on_cancel: C,
) -> impl IntoView
where
    C: Fn() + Copy + 'static,
{
    let (input, set_input) = create_signal(cx, "".to_owned());

    let options = move || query.call(input.get());

    let g_keyboard_event: GlobalKeyboardEvent = expect_context::<GlobalKeyboardEvent>(cx);
    create_effect(cx, move |_old| {
        if let Some(e) = g_keyboard_event.read_signal.get() {
            if show_when.get_untracked() && e.key().as_str() == "Escape" {
                on_cancel();
            }
        }
    });

    view! { cx,
        <Modal show_when=show_when class="quicksearch-modal">
            <ModalHeader>
                <Input
                    get=input
                    set=move |v| set_input.set(v)
                    label="Search"
                    class="search-input"
                    should_be_focused=show_when
                    prepend=view! {cx, ""}.into_view(cx)
                />
            </ModalHeader>
            <ModalBody>
                <leptonic-quicksearch-results>
                    { move || options().into_iter().map(|option| view! {cx,
                        <leptonic-quicksearch-result on:click=move |_| {
                                option.on_select.call(());
                                on_cancel();
                            }>
                            { option.view.call(cx) }
                        </leptonic-quicksearch-result>
                    }).collect_view(cx) }
                </leptonic-quicksearch-results>
            </ModalBody>
            <ModalFooter>
                <ButtonWrapper>
                    <Button on_click=move |_| (on_cancel)() color=ButtonColor::Secondary>"Cancel"</Button>
                </ButtonWrapper>
            </ModalFooter>
        </Modal>
    }
}
