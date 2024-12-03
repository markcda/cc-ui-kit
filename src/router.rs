use cc_utils::results::CResult;

/// Получение адреса и порта сервера
pub fn get_host() -> CResult<String> {
  let server_host = web_sys::window()
    .ok_or::<String>("Не удалось получить параметры окна браузера.".into())?
    .document()
    .ok_or::<String>("Не удалось получить параметры документа браузера.".into())?
    .location()
    .ok_or::<String>("Не удалось получить информацию о сервере из браузера.".into())?
    .host()
    .map_err(|e| format!("Ошибка получения информации о хосте: {:?}", e))?
    .to_string();
  Ok(server_host)
}

/// Получение протокола сервера (HTTP/HTTPS: "http:"/"https:")
pub fn get_protocol() -> CResult<String> {
  let server_proto = web_sys::window()
    .ok_or::<String>("Не удалось получить параметры окна браузера.".into())?
    .document()
    .ok_or::<String>("Не удалось получить параметры документа браузера.".into())?
    .location()
    .ok_or::<String>("Не удалось получить информацию о сервере из браузера.".into())?
    .protocol()
    .map_err(|e| format!("Ошибка получения информации о протоколе: {:?}", e))?
    .to_string();
  Ok(server_proto)
}

/// Получение местонахождения
pub fn get_path() -> CResult<String> {
  let path = web_sys::window()
    .ok_or::<String>("Не удалось получить параметры окна браузера.".into())?
    .document()
    .ok_or::<String>("Не удалось получить параметры документа браузера.".into())?
    .location()
    .ok_or::<String>("Не удалось получить информацию о сервере из браузера.".into())?
    .pathname()
    .map_err(|e| format!("Ошибка получения информации о пути страницы: {:?}", e))?
    .to_string();
  Ok(path)
}
