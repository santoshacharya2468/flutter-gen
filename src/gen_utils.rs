use std::{collections::HashMap, fs};
pub fn create_bloc(name:String){
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
            create_file(full_path)
        }
     }
}
pub fn create_folder(path:String){
    fs::create_dir(path).unwrap();
}
pub fn create_file(file:String){
    let file_type=file.split("_").last().unwrap();
    println!("Creating file of type {file_type}");
    fs::File::create(file.as_str()).unwrap();
}
pub fn create_repository(name:String){
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
            create_file(full_path)
        }
     }
}
pub fn create_module_dir(name:String){
     fs::create_dir_all(format!("lib/features/{name}")).unwrap();
}
pub fn create_module(name:String){
  create_module_dir(name.clone());
   create_bloc(name.clone());
   create_repository(name);

}