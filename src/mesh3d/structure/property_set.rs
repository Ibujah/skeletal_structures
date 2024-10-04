use anyhow::Result;

use std::collections::HashMap;

use ply_rs::ply::{Addable, DefaultElement, ElementDef, Property, PropertyDef, PropertyType};

/// Property set: save a set of properties attributed to a vertex, face, edge, ...
#[derive(Clone)]
pub struct PropertySet {
    /// Describe properties type (i.e. color, label, ...)
    property_type: HashMap<String, (PropertyType, Property)>,

    /// Property values
    property_value: HashMap<String, Vec<Property>>,

    /// Header name
    header_name: String,

    /// Number of elements
    nb_elements: usize,
}

impl PropertySet {
    /// Property set constructor
    pub fn new(name: &str, nb_elements: usize) -> PropertySet {
        PropertySet {
            property_type: HashMap::new(),
            property_value: HashMap::new(),
            header_name: name.to_string(),
            nb_elements,
        }
    }

    /// Adds a property with a default value
    pub fn add_property(
        &mut self,
        property_name: String,
        property_type: PropertyType,
        property_default: Property,
    ) -> () {
        let default_properties = vec![property_default.clone(); self.nb_elements];
        self.property_type.insert(
            property_name.clone(),
            (property_type, property_default.clone()),
        );
        self.property_value
            .insert(property_name.clone(), default_properties);
    }

    /// Push new element
    pub fn push_element(&mut self) -> () {
        self.nb_elements += 1;
        for (key, vec) in self.property_value.iter_mut() {
            vec.push(self.property_type[key].1.clone());
        }
    }

    /// Pop last element
    pub fn pop_element(&mut self) -> () {
        self.nb_elements -= 1;
        for (_, vec) in self.property_value.iter_mut() {
            vec.pop();
        }
    }

    /// Sets a property
    pub fn set_property_value(
        &mut self,
        ind_element: usize,
        property_name: String,
        property_value: Property,
    ) -> Result<()> {
        if self.nb_elements < ind_element {
            return Err(anyhow::Error::msg("Index out of bounds"));
        }
        self.property_value
            .get_mut(&property_name)
            .ok_or(anyhow::Error::msg(
                "No property ".to_owned() + property_name.as_str() + " in vertex properties",
            ))?[ind_element] = property_value;

        Ok(())
    }

    /// Gets a property
    pub fn get_property_value(
        &self,
        ind_element: usize,
        property_name: String,
    ) -> Result<Property> {
        if self.nb_elements < ind_element {
            return Err(anyhow::Error::msg("Index out of bounds"));
        }

        let vec = self
            .property_value
            .get(&property_name)
            .ok_or(anyhow::Error::msg(
                "No property ".to_owned() + property_name.as_str() + " in vertex properties",
            ))?;

        Ok(vec[ind_element].clone())
    }

    /// swap two indices
    pub fn swap_indices(&mut self, ind_element1: usize, ind_element2: usize) -> Result<()> {
        if self.nb_elements < ind_element1 || self.nb_elements < ind_element2 {
            return Err(anyhow::Error::msg("Index out of bounds"));
        }

        for (_, vec) in self.property_value.iter_mut() {
            vec.swap(ind_element1, ind_element2);
        }

        Ok(())
    }

    /// Get ply header element
    pub fn get_header_element(&self) -> ElementDef {
        let mut header_element = ElementDef::new(self.header_name.clone());
        for (key, (prop, _)) in self.property_type.iter() {
            header_element
                .properties
                .add(PropertyDef::new(key.clone(), prop.clone()));
        }
        header_element
    }

    /// Get ply payload element
    pub fn get_payload_element(&self) -> Vec<DefaultElement> {
        let mut payload_element = Vec::new();

        let nb_vertices = self
            .property_value
            .values()
            .max_by_key(|v| v.len())
            .unwrap()
            .len();

        for i in 0..nb_vertices {
            let mut vertex = DefaultElement::new();
            for (key, vec) in self.property_value.iter() {
                let value = vec[i].clone();
                vertex.insert(key.clone(), value.clone());
            }
            payload_element.push(vertex);
        }

        payload_element
    }
}
