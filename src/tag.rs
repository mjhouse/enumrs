use syn::{Meta, Attribute};
use evalexpr::*;

#[derive(Debug,Clone)]
pub struct Tag
{
    pub name: String,
    pub variant: String,
    pub expression: String,
    pub value: Option<Value>
}

impl Tag 
{
    pub fn is_type(&self, other: &Tag) -> bool {
        match ( &self.value, &other.value )
        {
            (Some(Value::String(_)),  Some(Value::String(_)))  |
            (Some(Value::Boolean(_)), Some(Value::Boolean(_))) |
            (Some(Value::Int(_)),     Some(Value::Int(_)))     |
            (Some(Value::Float(_)),   Some(Value::Float(_)))   |
            (Some(Value::Tuple(_)),   Some(Value::Tuple(_))) => true,
            _ => false
        }
    }

    pub fn is_evaluated(&self) -> bool {
        self.value.is_some()
    }

    pub fn evaluate(&mut self, context: &HashMapContext<DefaultNumericTypes>) -> Option<Value>
    {
        if self.value.is_none() {
            let result = eval_with_context(
                self.expression.as_str(), 
                context
            );
            self.value = result.ok();
        }
        self.value.clone()
    }

    pub fn parse(variant: String, attribute: &Attribute) -> Result<Self,()>
    {
        if let Meta::List(list) = attribute.meta.clone()
        {
            // collect name and value tokens
            let (name_vec,values_vec): (Vec<_>, Vec<_>) = list.tokens
                .into_iter()
                .map(|t| t.to_string())
                .filter(|s| s != ",")
                .filter(|s| !s.is_empty())
                .enumerate()
                .partition(|(i,_)| i == &0);

            // make sure we have a name
            if name_vec.len() == 1 {

                // get and clean the name
                let name = name_vec[0]
                    .1
                    .clone()
                    .trim()
                    .to_string();

                // get and clean the value
                let mut expression = values_vec
                    .into_iter()
                    .map(|(_,v)| v)
                    .collect::<String>()
                    .trim()
                    .to_string();

                // default value is 'true'
                if expression.is_empty() {
                    expression = String::from("true");
                }

                // return a valid tag context
                return Ok(Tag {
                    name: name,
                    variant: variant, 
                    expression: expression,
                    value: None
                });
            }
        }

        Err(())
    }
}