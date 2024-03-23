use std::{collections::HashMap, fs, io::Write};
use serde_json::Value;
use crate::{gen_command::CreateArgs, req_res:: serde_value_to_dart_class};
pub fn create_bloc(args:&CreateArgs){
    let name =&args.name;
    let base_dir=format!("lib/features/{name}/presentation");
    let bloc_dir=format!("{base_dir}/bloc");
    let widget_dir=format!("{base_dir}/widget");
    let screen_dir= format!("{base_dir}/screen");
    let dirs=vec![base_dir.clone(),bloc_dir.clone(),widget_dir.clone(),screen_dir.clone()];
    for dir in dirs{
        create_folder(dir);
    }
    let  mut  folder_files=HashMap::<String,Vec<String>>::new();
     folder_files.insert(bloc_dir, vec![format!("{name}_bloc.dart"),format!("{name}_event.dart"),format!("{name}_state.dart")]);
     folder_files.insert(screen_dir, vec![format!("{name}_screen.dart")]);
     folder_files.insert(widget_dir, vec![format!("{name}_widget.dart")]);
     for (key,value) in folder_files{
        for file in value{
            let full_path=format!("{key}/{file}");
            create_file(full_path,args);
        }
     }
}
pub fn create_folder(path:String){
    fs::create_dir(path).unwrap();
}
pub fn create_file(file:String,args:&CreateArgs){
    let file_type=file.split("_").last().unwrap();
    let mut file_content="//auto generated\n".to_string();
    match  file_type {
        "response.dart"=>{
            if args.res.is_some(){
                let value:&Value=&serde_json::from_str(args.res.as_ref().unwrap().as_str()).unwrap();
               file_content=serde_value_to_dart_class(value.as_object().unwrap(), format!("{}Response",args.name).as_str());
            }
        },
        "request.dart"=>{
            if args.req.is_some(){
                let value:&Value=&serde_json::from_str(args.req.as_ref().unwrap().as_str()).unwrap();
               file_content=serde_value_to_dart_class(value.as_object().unwrap(), format!("{}Request",args.name).as_str());
            }
        },
        "repository.dart"=>{
           
        }
        _=>{}
    }
    let mut  file=fs::File::create(file.as_str()).unwrap();
    let _=file.write(file_content.as_bytes());
}
pub fn create_repository(args:&CreateArgs){
    let name =&args.name;
    let base_dir=format!("lib/features/{name}/data");
    let repo_dir= format!("{base_dir}/repository");
    let model_dir=format!("{base_dir}/model",);
    let request_model_dir=format!("{base_dir}/model/request",);
    let response_model_dir=format!("{base_dir}/model/response",);
    let dirs=vec![base_dir.clone(),repo_dir.clone(),model_dir,response_model_dir.clone(),request_model_dir.clone()];
    for dir in dirs{
        create_folder(dir);
    }
    let  mut  folder_files=HashMap::<String,Vec<String>>::new();
     folder_files.insert(repo_dir, vec![format!("{name}_repository.dart"),format!("i_{name}_repository.dart")]);
     folder_files.insert(response_model_dir, vec![format!("{name}_response.dart"),format!("{name}_response.g.dart")]);
     folder_files.insert(request_model_dir, vec![format!("{name}_request.dart"),format!("{name}_request.g.dart")]);
     for (key,value) in folder_files{
        for file in value{
            let full_path=format!("{key}/{file}");
            create_file(full_path,args);
        }
     }
}
pub fn create_module_dir(name:String){
     fs::create_dir_all(format!("lib/features/{name}")).unwrap();
}
pub fn create_module(args:CreateArgs){
    let arg=&args;
    let name=&arg.name;
   create_module_dir(name.to_string());
   create_bloc(&arg);
   create_repository(&arg);

}