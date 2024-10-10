use actix_web::{web, App, HttpServer, HttpResponse, Error, HttpRequest, dev::ServiceRequest, dev::Service, dev::ServiceResponse, middleware, Responder};
use pyo3::prelude::*;
// use pyo3::types::IntoPyDict;
use std::collections::VecDeque;

#[pyclass]
pub struct SpeechVec {
    #[pyo3(get, set)]
    speech_string: String,
    queue: VecDeque<String>,
    #[pyo3(get, set)]
    max_size: usize, // 最大队列长度
}

#[pymethods]
impl SpeechVec {
    #[new]
    fn new(max_size: usize) -> Self {
        run_server();
        SpeechVec {
            speech_string: String::new(),
            queue: VecDeque::new(),
            max_size,
        }
    }

    // 向队列中添加元素
    fn push(&mut self, element: String) {
        self.speech_string = element; 
        if self.queue.len() >= self.max_size {
            self.queue.pop_front(); // 如果队列已满，移除最前面的元素
        }
        // self.queue.push_back(self.speech_string); // 添加新元素到队列末尾
        self.queue.push_back(std::mem::take(&mut self.speech_string));
    }
    
    fn get_all(&self) -> Vec<String> {
        self.queue.iter().cloned().collect() // 返回队列中的元素作为向量
    }
}
    async fn get_uuid() -> HttpResponse {
    HttpResponse::Ok().body("hello world".to_string())
}


pub    fn run_server() {
        let handle = tokio::spawn(async {
            let _ = serve(); 
        });
}




#[actix_web::main]
async fn serve() -> std::io::Result<()> {
    // 启动 HTTP 服务器
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(get_uuid))
    })
    .bind(("127.0.0.1", 6838))? 
    .run()
    .await
}


#[pyfunction]
pub fn speak(input_message: &str) -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python module (example.py should be in the working directory)
        let ui = PyModule::import_bound(py, "speech")?;
        let _message: PyObject = ui.getattr("speak")?.call1((input_message, ))?.into();

        Ok(())
    })
}

#[pymodule]
pub fn speech(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(speak, m)?)?;
    Ok(())
}

#[cfg(test)]
mod test{

#[tokio::test]
        async fn test_speech_vec() {
            use super::*;
    let mut queue = SpeechVec::new(3); // 队列最大长度为3

    // 添加元素
    queue.push("element1".to_string());
    queue.push("element2".to_string());
    queue.push("element3".to_string());

    println!("Queue after 3 pushes: {:?}", queue.get_all());

    // 再次添加元素，超过长度时会自动移除前端元素
    queue.push("element4".to_string());

    println!("Queue after pushing element4: {:?}", queue.get_all());
}

}