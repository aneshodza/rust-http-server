/// This struct is used to store the attributes of the incoming http request
pub struct HttpObject {
    request: String,
    accept: String,
    // accept_encoding: String,
}

/// This is the implementation of the HttpResponse. It gives the user methods to more easily
/// interact with the HttpResponse struct by giving helper functions
impl HttpObject {
    /// This Initializes a new `HttpObject`
    ///
    /// # Returns
    ///
    /// It returns the newly created `HttpObject`
    pub fn new(request: String, accept: String) -> HttpObject {
        HttpObject {
            request,
            accept,
            // Initialize accept_encoding or any other fields as needed
        }
    }

    /// This function checks if the incoming request is an HTTP request
    /// 
    /// # Returns
    ///
    /// Returns a boolean value that is true if the request is an HTTP request
    pub fn is_http(&self) -> bool {
        self.request.split_whitespace().collect::<Vec<&str>>()[2] == "HTTP/1.1"
    }

    /// This function returns the request path of the incoming HTTP request
    ///
    /// # Returns
    ///
    /// Returns a `&str` of the request path
    pub fn request_path(&self) -> &str {
        self.request.split_whitespace().collect::<Vec<&str>>()[1]
    }

    /// This function returns the accept attribute of the incoming HTTP request
    ///
    /// # Returns
    ///
    /// Returns a `&str` of the accept attribute
    pub fn weighted_mimes(&self) -> Result<Vec<(String, f32)>, String> {
        if !self.accept_is_valid() {
            return Err("No accept attribute found".to_string());
        }

        let types = self.extract_types();
        let weights = self.extract_weights(types.len());

        let weighted_types = self.connect_types_to_weights(&types, &weights);

        Ok(weighted_types)
    }

    /// This function checks if the accept attribute is valid
    ///
    /// # Returns
    ///
    /// It returns a `bool` that is true if the accept attribute is valid
    fn accept_is_valid(&self) -> bool {
        if !self.accept.len() == 0 { return false; }
        if !(self.accept.contains("Accept:") || self.accept.contains("accept:")) { return false };

        true
    }

    /// This function extracts the weights from the accept attribute
    ///
    /// # Parameters
    ///
    /// - `amount`: This is the amount of weights that are expected to be extracted. It is used to
    /// fill the weights with `1.0` if there are no weights
    ///
    /// # Returns
    ///
    /// It returns a `Vec<f32>` of the weights. They are ordered as they appear in the accept. If
    /// there are no weights it defaults to `1.0` for every type
    fn extract_weights(&self, amount: usize) -> Vec<f32> {
        let weights = self.accept.split("q=").collect::<Vec<&str>>();
        if weights.len() == 1 {
            return vec![1.0; amount]
        }
        let mut handled_weights: Vec<f32> = Vec::new();
        for i in 1..weights.len() {
            handled_weights.push(weights[i][..3].parse::<f32>().unwrap());
        }

        handled_weights
    }

    /// This functions connects the given mime types to the passed weights
    ///
    /// # Parameters
    ///
    /// - `types`: This is a `Vec<&str>` of the types
    /// - `weights`: This is a `Vec<f32>` of the weights
    ///
    /// # Returns
    ///
    /// It returns a `Vec<(String, f32)>` of the types and their weights
    fn connect_types_to_weights(&self, types: &Vec<&str>, weights: &Vec<f32>) -> Vec<(String, f32)> {
        let mut weighted_types: Vec<(String, f32)> = Vec::new();
        for i in 0..types.len() {
            let type_vec = types[i].split(",").collect::<Vec<&str>>();
            for j in 0..type_vec.len() {
                weighted_types.push((type_vec[j].to_string(), weights[i]));
            }
        }
        weighted_types
    }

    /// This function extracts the types from the accept attribute
    ///
    /// # Returns
    ///
    /// It returns a `Vec<&str>` of the types. They are ordered as they appear in the accept
    fn extract_types(&self) -> Vec<&str> {
        let formatted_accept = self.accept.split(": ").collect::<Vec<&str>>()[1];
        let mut types = formatted_accept.split(";").collect::<Vec<&str>>();
        if types.len() == 1 {
            return vec![formatted_accept]
        }
        types.pop();
        for i in 1..types.len() {
            types[i] = &types[i][6..];
        }
        types
    }
}
