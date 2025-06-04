use std::{cmp::Ordering, collections::HashSet, rc::Rc};
use web_sys::HtmlInputElement;
use yew::{prelude::*, virtual_dom::AttrValue};
use gloo::timers::callback::Timeout;
use super::AutocompleteInput;
use crate::{best_build_search::helper_enums::SearchReq, website::build_reqs_input::BuildReqsInput, wynn_data::items::{self, Type, WynnItem}};

#[derive(Properties, PartialEq)]
pub struct ItemInputProps{
    #[prop_or_default]
    /// Applies the `class` html tag to the wrapper div
    pub class: &'static str,
    #[prop_or("".into())]
    /// Applies the `id` html tag to the wrapper div
    pub id: AttrValue,
    /// What type (category) of wynncraft item this input is used for
    pub item_type: Type,
    #[prop_or(1)]
    /// Minimum number of item input this component can have
    pub min_inputs: usize,
    #[prop_or_default]
    /// Callback to retrieve items when this component loses focus
    pub on_leave: Callback<(Type, Vec<WynnItem>)>,
    #[prop_or_default]
    pub start_value: Vec<WynnItem>,
}
pub enum ItemInputMsg{
    OnFocus,
    InputChanged(usize, (Option<usize>, String)),
    OpenSearch,
    UpdateSearch(String),
    Search,
    OnBlur,
    OnLeave
}
pub struct ItemInput{
    focused: bool,
    selection: Vec<WynnItem>,
    items: Vec<WynnItem>,
    item_names: Rc<Vec<String>>,
    item_rarities: Rc<Vec<String>>,
    searching: bool,
    searching_val: u32,
    unfocus_handle: Option<Timeout>
}
impl Component for ItemInput{
    type Message = ItemInputMsg;

    type Properties = ItemInputProps;

    fn create(ctx: &Context<Self>) -> Self {
        let items: Vec<WynnItem> = items::iter().filter(|itm| itm.get_type()==ctx.props().item_type).collect();
        // items.sort_by(|a, b| a.name().cmp(b.name()));

        ItemInput{focused: false, unfocus_handle: None, 
            selection: if ctx.props().start_value.is_empty(){vec![WynnItem::NULL; ctx.props().min_inputs]} else {let mut temp = ctx.props().start_value.clone(); temp.push(WynnItem::NULL); temp}, 
            item_names: items.iter().map(|itm| itm.name().to_string()).collect::<Vec<String>>().into(), 
            item_rarities: items.iter().map(|itm| itm.get_tier().to_string()).collect::<Vec<String>>().into(), items, searching: false, searching_val: 200}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg{
            ItemInputMsg::OnFocus => {
                self.focused=true;
            },
            ItemInputMsg::InputChanged(input_idx, (option_idx, _)) => {
                match option_idx{
                    Some(id) => {
                        self.selection[input_idx]=self.items[id];
                        if input_idx==self.selection.len()-1{
                            self.selection.push(WynnItem::NULL);
                        }
                    },
                    None => {
                        self.selection[input_idx]=WynnItem::NULL;
                    }
                }
            },
            ItemInputMsg::OpenSearch => {
                self.searching = true;
            },
            ItemInputMsg::UpdateSearch(s) => {
                self.searching_val = s.parse().unwrap_or(200);
            },
            ItemInputMsg::Search => {
                // lazy code; doesn't remove duplicates
                self.selection.extend(self.items.iter()
                    .filter(|itm| itm.get_lvl() >= self.searching_val)
                );

                // this is for when i get around do adding full Atr list as possible item search reqs
                // self.selection.extend(self.items.iter()
                //     .filter(|itm| reqs.iter()
                //         .all(|(ord, req)| 
                //             match req { 
                //                 SearchReq::Stat(atr, val) => itm.get_ident(*atr).unwrap_or_default().cmp(val) == *ord, 
                //                 SearchReq::Calc(_calc, _val) => true,
                //             }
                //         )));
            },
            ItemInputMsg::OnBlur => {
                // onblur always gets called when nested `<input/>`'s lose focus, even if the focus was redirected to another nested input type. 
                // i want OnLeave to *only* get called when all the nested components lose focus (none of the nested components have focus)
                // to prevent this, i set a timeout to delay OnLeave from being called until after OnFocus gets an opportunity to get called again,
                // preventing false 'onblur' calls
                if self.focused{
                    self.focused = false;
                    let link = ctx.link().clone();
                    self.unfocus_handle = Some(Timeout::new(0, move || link.send_message(ItemInputMsg::OnLeave)));    
                }else{
                    return false
                }
            },
            ItemInputMsg::OnLeave => {
                if !self.focused{
                    self.searching = false;
                    // filter out 'null' items and remove them from the selection, in addition to removing duplicates
                    let mut found: HashSet<u32> = HashSet::new();
                    self.selection.retain(|itm| !itm.is_null() && found.insert(itm.id()));
                    // emit callback using the selection
                    ctx.props().on_leave.emit((ctx.props().item_type,self.selection.clone().into_iter().filter(|itm| !itm.is_null()).collect::<Vec<WynnItem>>()));
                    self.selection.resize(ctx.props().min_inputs.max(self.selection.len()+1), WynnItem::NULL);
                }else{
                    return false
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let div_content = html!{
            <>
                <h3>{ctx.props().item_type}</h3>
                <img src={format!("images/wynn-{}.png",ctx.props().item_type.to_string().to_lowercase())}/>
                <button class="search-button" onclick={link.callback(|_| ItemInputMsg::OpenSearch)}>
                    <svg xmlns="http://www.w3.org/2000/svg" x="0px" y="0px" width="30" height="30" viewBox="0 0 24 24">
                        <circle cx="9" cy="9" r="5"/>
                        <path d="M 9 2 C 5.1458514 2 2 5.1458514 2 9 C 2 12.854149 5.1458514 16 9 16 C 10.747998 16 12.345009 15.348024 13.574219 14.28125 L 14 14.707031 L 14 16 L 20 22 L 22 20 L 16 14 L 14.707031 14 L 14.28125 13.574219 C 15.348024 12.345009 16 10.747998 16 9 C 16 5.1458514 12.854149 2 9 2 z M 9 4 C 11.773268 4 14 6.2267316 14 9 C 14 11.773268 11.773268 14 9 14 C 6.2267316 14 4 11.773268 4 9 C 4 6.2267316 6.2267316 4 9 4 z"></path>
                    </svg>
                </button>
                if self.searching{
                    <div class="input-search">
                        {"lvl > "}
                        <input 
                            oninput={
                                link.callback(|event: InputEvent| {
                                    let input: HtmlInputElement = event.target_unchecked_into();
                                    ItemInputMsg::UpdateSearch(input.value())
                                })
                            }
                            onkeypress={
                                link.callback(|key:KeyboardEvent| {
                                    if key.char_code()==13 {
                                        ItemInputMsg::Search
                                    } else {
                                        ItemInputMsg::OpenSearch
                                    }
                                })
                            }
                        />
                    </div>
                }
                <div class="item-input-list">
                    {self.selection.iter().enumerate().map(|(i,v)|{
                        html!{
                            <AutocompleteInput class={format!("item-list-input")} stick_to_input=true placeholder={format!("Insert {}",ctx.props().item_type)} value={if v.is_null(){String::new()}else{v.name().to_string()}} unfocus_delay=250 options = {&self.item_names} on_leave = {link.callback(move |v| ItemInputMsg::InputChanged(i, v))} on_select = {link.callback(move |(idx,s)| ItemInputMsg::InputChanged(i, (Some(idx),s)))} options_classes ={&self.item_rarities}/>
                        }
                    }).collect::<Html>()}
                </div>
            </>
        };

        if ctx.props().id.is_empty(){
            html!{
                <div class={format!("item-input-wrapper {}",ctx.props().class)} onfocus={link.callback(|_| ItemInputMsg::OnFocus)} onblur={link.callback(|_| ItemInputMsg::OnBlur)}>
                    {div_content}
                </div>
            }
        }else{
            html!{
                <div id={ctx.props().id.clone()} class={format!("item-input-wrapper {}",ctx.props().class)} onfocus={link.callback(|_| ItemInputMsg::OnFocus)} onblur={link.callback(|_| ItemInputMsg::OnBlur)}>
                    {div_content}
                </div>
            }
        }
    }
}