use leptos::*;
use leptos_struct_table::*;

const ROW_HEIGHT: usize = 30;
const ROW_HEIGHT_HALF: usize = ROW_HEIGHT / 2;

wrapper_render_fn!(
    /// g
    GRenderer,
    g,
);

#[allow(unused_variables, non_snake_case)]
pub fn SvgRowRenderer<Row>(
    class: Signal<String>,
    row: Row,
    index: usize,
    selected: Signal<bool>,
    on_select: EventHandler<web_sys::MouseEvent>,
    on_change: EventHandler<ChangeEvent<Row>>,
) -> impl IntoView
where
    Row: TableRow + Clone + 'static,
{
    let transform = format!("translate(0, {})", (index + 1) * ROW_HEIGHT);

    view! {
        <g
            class=class
            transform=transform
            on:click=move |mouse_event| on_select.run(mouse_event)
        >
            <line
                x1="5"
                y1="0"
                x2="150"
                y2="0"
                stroke-width="1px"
                stroke="black"
                opacity="0.1"
            ></line>
            {row.render_row(index, on_change)}
        </g>
    }
}

#[allow(non_snake_case)]
pub fn SvgErrorRowRenderer(err: String, index: usize, _col_count: usize) -> impl IntoView {
    let transform = transform_from_index(index, 0);

    view! {
        <g transform=transform>
            <text x="0" y=ROW_HEIGHT_HALF dominant-baseline="central">
                {err}
            </text>
        </g>
    }
}

#[allow(non_snake_case)]
pub fn SvgLoadingRowRenderer(
    class: Signal<String>,
    _cell_class: Signal<String>,
    inner_cell_class: Signal<String>,
    index: usize,
    _col_count: usize,
) -> impl IntoView {
    let transform = transform_from_index(index, 0);

    view! {
        <g class=class transform=transform>
            <text x="0" y=ROW_HEIGHT_HALF class=inner_cell_class dominant-baseline="central">
                Loading...
            </text>
        </g>
    }
}

#[component]
pub fn SvgHeadCellRenderer<F>(
    /// The class attribute for the head element. Generated by the classes provider.
    #[prop(into)]
    class: Signal<String>,
    /// The class attribute for the inner element. Generated by the classes provider.
    #[prop(into)]
    inner_class: String,
    /// The index of the column. Starts at 0 for the first column. The order of the columns is the same as the order of the fields in the struct.
    index: usize,
    /// The sort priority of the column. `None` if the column is not sorted. `0` means the column is the primary sort column.
    #[prop(into)]
    sort_priority: Signal<Option<usize>>,
    /// The sort direction of the column. See [`ColumnSort`].
    #[prop(into)]
    sort_direction: Signal<ColumnSort>,
    /// The event handler for the click event. Has to be called with [`TableHeadEvent`].
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableHeadEvent) + 'static,
{
    let style = move || {
        let sort = match sort_direction() {
            ColumnSort::Ascending => "--sort-icon: '▲';",
            ColumnSort::Descending => "--sort-icon: '▼';",
            ColumnSort::None => "--sort-icon: '';",
        };

        let priority = match sort_priority() {
            Some(priority) => format!("--sort-priority: '{}';", priority + 1),
            None => "--sort-priority: '';".to_string(),
        };

        format!("{} {}", sort, &priority)
    };

    let transform = transform_from_index(index, 0);

    view! {
        <g
            class=class
            transform=transform
            on:click=move |mouse_event| on_click(TableHeadEvent {
                index,
                mouse_event,
            })

            style=style
        >
            <text x="0" y=ROW_HEIGHT_HALF class=inner_class dominant-baseline="central">
                {children()}
            </text>
        </g>
    }
}

#[component]
#[allow(unused_variables)]
pub fn SvgTextCellRenderer<T, F>(
    class: String,
    #[prop(into)] value: MaybeSignal<T>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    T: IntoView + Clone + 'static,
    F: Fn(T) + 'static,
{
    let x = x_from_index(index);

    view! {
        <text x=x y=ROW_HEIGHT_HALF class=class dominant-baseline="central">
            {value}
        </text>
    }
}

#[component]
#[allow(unused_variables)]
pub fn SvgPathCellRenderer<F>(
    #[prop(into)] class: String,
    #[prop(into)] value: MaybeSignal<String>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let transform = transform_from_index(index, 3);

    view! { <path transform=transform class=class d=value></path> }
}

fn transform_from_index(index: usize, y: usize) -> String {
    format!("translate({}, {y})", x_from_index(index))
}

fn x_from_index(index: usize) -> usize {
    5 + index * 100
}
