// use core::num::dec2flt::parse;
use std::collections::VecDeque;
use std::string::ToString;
use std::any::type_name;
use std::any::TypeId;
use regex::Regex;
// <ID>:<SIZE>:{[<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>]...}

#[derive(Debug, Clone)]
pub enum QType {
    DEFAULT,
    QInt,
    QFloat,
    QString,
    QArray,
    QJson
}

pub fn QTypeToValue(_qType : QType) -> i32{
    match _qType {
        QType::DEFAULT => return 0,
        QType::QInt => return 1,
        QType::QFloat => return 2,
        QType::QString => return 3,
        QType::QArray => return 4,
        _ => return -1
    };
    return -1
}

pub struct QTuple<T> {
    field: (i32, i64, f64, String, Vec<T>)
}

pub struct QUnpacked<T> {
    data : Vec<QTuple<T>>
}

impl<T: std::fmt::Display> QTuple<T> {
    pub fn new_zero() -> Self {
        return QTuple { field: (0, 0, 0.0, "".to_string(), Vec::new()) };
    }
    pub fn new(qinteger : i64, qfloat : f64, qstring : String, qvec : Vec<T>) -> Self {
        return QTuple { field: (0, qinteger, qfloat, qstring, qvec) }
    }
}

// pub fn deserialize_qvalue(q_type: QType, buffer: String) -> QTuple<T> {
//     match q_type {
//         QType::DEFAULT => QTuple::new_zero(),
//         QType::QInt => {
//             match buffer.parse::<i64>() {
//                 Ok(num) => QTuple::new(num, 0.0, "".to_string(), Vec::new()),
//                 Err(_) => {
//                     println!("Convert Error!");
//                     QTuple::new_zero()
//                 }
//             }
//         },
//         QType::QFloat => {
//             match buffer.parse::<f64>() {
//                 Ok(num) => QTuple::new(0, num, "".to_string(), Vec::new()),
//                 Err(_) => {
//                     println!("Convert Error!");
//                     QTuple::new_zero()
//                 }
//             }
//         },
//         QType::QString => QTuple::new(0, 0.0, buffer, Vec::new()),
//         QType::QArray => {
//             // Handle QArray case
//             // You need to implement this part
//             unimplemented!()
//         },
//         _ => {
//             // Handle other cases
//             // You need to decide what to return here
//             QTuple::new_zero()
//         }
//     }
// }



#[derive(Debug, Clone)]
pub struct QValue 
{
    qType : QType,
    meta_data : usize,
    buffer : String
}

impl QValue 
{
    pub fn new_zero(_qType : QType) -> QValue {
        return QValue { qType: _qType, meta_data: 0, buffer: String::new() }
    }

    pub fn new(_type : QType, _meta_data : usize, _data : String) -> QValue {
        return QValue { qType: _type, meta_data: _meta_data, buffer : _data }
    }

    pub fn convert() -> String {
        return "".to_string()
    }
}



pub trait QAction {
    fn Initialize(&mut self);
    fn get_value(&mut self) -> QValue;
    fn get_buffer(&mut self) -> String;
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QInteger 
{
    val : QValue,
    data : i64
}

impl QInteger {
    pub fn new(data : i64) -> QInteger {
        let mut ret = QInteger {val : QValue::new_zero(QType::QInt), data};
        ret.Initialize();
        return ret
    }
}

impl QAction for QInteger {
    fn Initialize(&mut self) {

        self.val.meta_data = 0; // QInteger not use meta data
        self.val.buffer = "[".to_owned() + &QTypeToValue(self.val.qType.clone()).to_string() + ":" + &self.val.meta_data.to_string() + ":" + &self.data.to_string() + "]"; 
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QFloat 
{
    val : QValue,
    data : f64
}

impl QFloat {
    pub fn new(data : f64) -> QFloat {
        let mut ret = QFloat {val : QValue::new_zero(QType::QFloat), data};
        ret.Initialize();
        return ret
    }
}

impl QAction for QFloat {
    fn Initialize(&mut self) {
        self.val.meta_data = 0; // QFloat not use meta data
        self.val.buffer = "[".to_owned() + &QTypeToValue(self.val.qType.clone()).to_string() + ":" + &self.val.meta_data.to_string() + ":" + &self.data.to_string() + "]"; 
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QString 
{
    val : QValue,
    data : String
}

impl QString {
    pub fn new(_data : String) -> QString {
        let mut ret = QString {val : QValue::new_zero(QType::QString), data : _data};
        ret.Initialize();
        return ret
    }
}

impl QAction for QString {
    fn Initialize(&mut self) {
        self.val.meta_data = self.data.len().clone(); // QString use meta data as length
        self.val.buffer = "[".to_owned() + &QTypeToValue(self.val.qType.clone()).to_string() + ":" + &self.val.meta_data.to_string() + ":" + &self.data + "]";         
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}

// [<TYPE>:<META_DATA>:<VALUE>]
pub struct QArray<T>
{
    val : QValue,
    elem_type : QType,
    data : Vec<T>
}

impl<T: std::fmt::Display> QArray<T> {
    pub fn new(_data: Vec<T>, _elem_type : QType) -> QArray<T> {
        let mut ret = QArray {
            val: QValue::new_zero(QType::QArray),
            elem_type: _elem_type,
            data: _data,
        };
        ret.Initialize();
        return ret;
    }
}

fn perform_action_by_type(value: &dyn std::fmt::Display) -> String {
    value.to_string()
}


// [<TYPE>:<META_DATA>:=<ELEM_TYPE>=<VALUE>,<VALUE>,<VALUE>,<VALUE>,<VALUE>, . . .]
impl<T: std::fmt::Display> QAction for QArray<T> {
    fn Initialize(&mut self) {
        self.val.meta_data = self.data.len(); // QArray uses meta data as length
    
        self.val.buffer = "[".to_owned() + &QTypeToValue(self.val.qType.clone()).to_string()
            + ":" + &self.val.meta_data.to_string() + ":" + "=" + &QTypeToValue(self.elem_type.clone()).to_string() + "=";
    
            for elem in &self.data {
                let _elem_buf = perform_action_by_type(elem);
                self.val.buffer += &_elem_buf;
                self.val.buffer += ",";
            }
            self.val.buffer.pop();
            
        self.val.buffer += "]";
    }

    fn get_value(&mut self) -> QValue {
        return self.val.clone()
    }

    fn get_buffer(&mut self) -> String {
        let mut ret = self.val.buffer.clone();
        return ret
    }
}


// <ID>:<SIZE>:{[<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>]...}
pub struct QMessage 
{
    id : i64, // <ID> for RPC
    size : usize, // <SIZE>
    data : Vec<String> // {[<TYPE>:<META_DATA>:<VALUE>]. . .}
}

impl QMessage {
    pub fn new (_id : i64, _size : usize, _data : Vec<String>) -> QMessage {
        return QMessage { id: _id, size : _size , data: _data }
    }

    pub fn get_id (&mut self) -> i64 {
        return self.id.clone()
    }

    pub fn get_size (&mut self) -> usize {
        return self.size.clone()
    }

    pub fn get_data (&mut self) -> Vec<String> {
        return self.data.clone()
    }
}

pub trait MessageBuilder
{
    fn message_build(self) -> QMessage;
    fn unpack_message(self, buffer : String) -> Self;
}


pub fn deseirialize(input: &str) -> Option<(u32, u32, String)> {

    let re = Regex::new(r"(\d+):(\d+):(.*)").unwrap();
    if let Some(captures) = re.captures(input) {

        let id: u32 = captures[1].parse().unwrap();
        let size: u32 = captures[2].parse().unwrap();
        let data = captures[3].to_owned(); 
        Some((id, size, data))
    } else {
        None
    }
}

pub fn extract_data(input: &str) -> Vec<String> {
    let re = Regex::new(r"\[[^\[\]]*\]").unwrap();
    re.find_iter(input)
        .map(|m| m.as_str().to_string())
        .collect()
}




// <ID>:<SIZE>:{[<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>][<TYPE>:<META_DATA>:<VALUE>]...}
pub fn seirialize(mut msg : QMessage) -> String {
    // id + data . . .
    
    let mut serialized = msg.get_id().to_string() + ":"
     + &msg.get_size().to_string() + ":" + "{";

     for elem in &msg.data {
        serialized += elem;
        
    } 

    serialized += "}";


    return serialized
}