use gloo_file::callbacks::FileReader;
use gloo_file::File;
use image::io::Reader;
use reqwest::multipart::Form;
use reqwest::{multipart, Client};
use std::collections::HashMap;
use web_sys::{DragEvent, Event, FileList, FormData, HtmlFormElement, HtmlInputElement};
use yew::{html, Callback, Component, Context, Html, TargetCast};

#[derive(Debug, Clone, Default)]
pub struct UploadFields {
    title: String,
    author: String,
    op: String,
    material: String,
    link: String,
    tags: String,
    filename: String,
    width: u32,
    height: u32,
}

impl UploadFields {
    fn fill(&mut self, data: FormData) {
        self.title = data
            .get("title")
            .as_string()
            .expect("Field failed to retrieve");
        self.author = data
            .get("author")
            .as_string()
            .expect("Field failed to retrieve");
        self.op = data
            .get("op")
            .as_string()
            .expect("Field failed to retrieve");
        self.material = data
            .get("material")
            .as_string()
            .expect("Field failed to retrieve");
        self.link = data
            .get("link")
            .as_string()
            .expect("Field failed to retrieve");
        self.tags = data
            .get("tags")
            .as_string()
            .expect("Field failed to retrieve");
    }
}

pub struct FileUpload {
    data: FormData,
    fields: UploadFields,
    file_data: Vec<u8>,
    form: Form,
    readers: HashMap<String, FileReader>,
}

pub enum UploadMsg {
    Files(Vec<File>),
    Loaded(String, Vec<u8>),
    Submit(FormData),
    None,
}

impl Component for FileUpload {
    type Properties = ();
    type Message = UploadMsg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: FormData::new().unwrap(),
            form: Form::default(),
            readers: HashMap::default(),
            fields: UploadFields::default(),
            file_data: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UploadMsg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();

                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo_file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(UploadMsg::Loaded(
                                file_name,
                                res.expect("Failed to read filed"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            UploadMsg::Loaded(file_name, data) => {
                self.file_data = data.clone();
                let dimensions = Reader::new(std::io::Cursor::new(data))
                    .with_guessed_format()
                    .unwrap()
                    .into_dimensions()
                    .unwrap();
                self.fields.width = dimensions.0;
                self.fields.height = dimensions.1;

                self.readers.remove(&file_name);
                self.fields.filename = file_name;
                true
            }
            UploadMsg::Submit(data) => {
                self.fields.fill(data);
                let file_data = self.file_data.clone();
                let fields = self.fields.clone();
                let part = multipart::Part::bytes(file_data.to_vec());

                // FIXME: This could be better :>
                let form = multipart::Form::new()
                    .text("title", fields.title)
                    .text("author", fields.author)
                    .text("op", fields.op)
                    .text("material", fields.material)
                    .text("link", fields.link)
                    .text("tags", fields.tags)
                    .text("filename", fields.filename)
                    .text("width", fields.width.to_string())
                    .text("height", fields.height.to_string())
                    .part("image", part);

                ctx.link().send_future(async move {
                    Client::new()
                        .post("http://localhost:7878/api/post_image")
                        .multipart(form)
                        .send()
                        .await
                        .unwrap();
                    UploadMsg::None
                });
                true
            }
            UploadMsg::None => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="post-upload">
                <form id="upload_form"
                    onsubmit={ctx.link().callback(move |event: web_sys::FocusEvent| {
                        event.prevent_default();
                        let form = event.target_unchecked_into::<HtmlFormElement>();
                        let data = FormData::new_with_form(&form).unwrap();
                        UploadMsg::Submit(data)
                    })}
                >
                    <label for="file-upload">
                        <div id="drop-container"
                            ondrop={ctx.link().callback(|event: DragEvent| {
                                event.prevent_default();
                                let files = event.data_transfer().unwrap().files();
                                Self::upload_files(files)
                            })}
                            ondragover={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                            ondragenter={Callback::from(|event: DragEvent| {
                                event.prevent_default();
                            })}
                        >
                            <i class="fa-file-upload"></i>
                            <p>{"Drag and drop image, or click to select image"}</p>
                        </div>
                    </label>
                    <label for="title">{"Title:"}</label><br/>
                    <input type="text" id="title" name="title" required=true/><br/>
                    <label for="author">{"Author:"}</label><br/>
                    <input type="text" id="author" name="author" required=true/><br/>
                    <label for="op">{"OP:"}</label><br/>
                    <input type="text" id="op" name="op"/><br/>
                    <label for="material">{"Material:"}</label><br/>
                    <input type="text" id="material" name="material" required=true/><br/>
                    <label for="link">{"Link:"}</label><br/>
                    <input type="text" id="link" name="link"/><br/>
                    <label for="tags">{"Tags:"}</label><br/>
                    <input type="text" id="tags" name="tags"/><br/>
                    <input
                        id="file-upload"
                        type="file"
                        accept="image/*,video/*"
                        multiple={false}
                        name="image"
                        required=true
                        onchange={ctx.link().callback(move |e: Event| {
                            let input = e.target_unchecked_into::<HtmlInputElement>();
                            Self::upload_files(input.files())
                    })}
                    /><br/>
                </form>
                <input type="submit"
                    form="upload_form"
                    value="submit"
                />
            </div>
        }
    }
}

impl FileUpload {
    fn upload_files(files: Option<FileList>) -> UploadMsg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        UploadMsg::Files(result)
    }
}
