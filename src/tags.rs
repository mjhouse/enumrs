use syn::Variant;
use evalexpr::*;
use crate::tag::Tag;

#[derive(Debug,Clone)]
pub struct Tags(pub Vec<Tag>);

impl Tags
{
    pub fn parse(variant: &Variant) -> Result<Self,String>
    {
        // create a context for tag evaluation
        let mut context = HashMapContext::<DefaultNumericTypes>::new();

        // get the name of the variant
        let name = variant.ident.to_string();

        // collect all tags from variant
        let mut tags = variant.attrs
            .iter()
            .map(|a| Tag::parse(name.clone(),a))
            .filter_map(Result::ok)
            .collect::<Vec<Tag>>();

        let mut progress = true;

        // this loop should iterate until either
        // all of the tags have been evaluated,
        // or it makes one full pass and no new
        // tags are evaluated.
        while progress {

            progress = false;

            // iterate through all unevaluated tags
            for tag in tags.iter_mut().filter(|t| !t.is_evaluated()) {

                // if it was evaluated add it to context
                if let Some(v) = tag.evaluate(&context)
                {
                    // set progress true so we continue iterating
                    progress = true;
                    context
                        .set_value(tag.name.clone(), v)
                        .or_else(|e| Err(format!("(variant:{}): {}",tag.name, e)))?;
                }
            }
        }        

        // filter out any unevaluated tags
        let (unevaluated,evaluated): (Vec<_>,Vec<_>) = tags
            .into_iter()
            .partition(|a| !a.is_evaluated());

        // fail due to invalid expressions in tag values
        if unevaluated.len() > 0 {
            for tag in unevaluated {
                return Err(format!(
                    "(variant:{}): invalid expression", tag.name
                ));
            }
        }

        // return the variant context
        Ok(Tags(evaluated))
    }
}