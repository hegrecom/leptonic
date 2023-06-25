use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageChangelog(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Changelog"</H1>

        <H2>"0.0.1"</H2>

        <P>"Initial release."</P>

        <H3>"Technical additions:"</H3>
        <ul>
            <li>"Callback types"</li>
            <li>"Global event listener contexts"</li>
            <li>"Root component"</li>
        </ul>

        <H3>"Added:"</H3>
        <ul>
            <li>"Skeleton component and styles"</li>
            <li>"Stack component and styles"</li>
            <li>"Grid component and styles"</li>
            <li>"Separator component and styles"</li>
            <li>"Tab components and styles"</li>
            <li>"Collapsible components and styles"</li>
            <li>"Button component and styles"</li>
            <li>"Input component and styles"</li>
            <li>"Date selector component and styles"</li>
            <li>"Slider component and styles"</li>
            <li>"Select component and styles"</li>
            <li>"Toggle component and styles"</li>
            <li>"Alert component and styles"</li>
            <li>"Toast component and styles"</li>
            <li>"Modal components and styles"</li>
            <li>"Progress component and styles"</li>
            <li>"Chip component and styles"</li>
            <li>"Icon component and styles"</li>
            <li>"Link component and styles"</li>
            <li>"Anchor component and styles"</li>
            <li>"Typography components and styles"</li>
            <li>"Transition components and styles"</li>
        </ul>
    }
}
