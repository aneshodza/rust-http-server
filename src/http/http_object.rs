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
    /// TODO: Extract into multiple methods and handle no weights
    pub fn accepted_mime_with_weight(&self) -> Vec<(String, f32)> {
        // self.accept.split_whitespace().collect::<Vec<&str>>()[1]
        let weights = self.accept.split("q=").collect::<Vec<&str>>();
        let mut handled_weidhts: Vec<f32> = Vec::new();
        for i in 1..weights.len() {
            handled_weidhts.push(weights[i][..3].parse::<f32>().unwrap());
        }

        let formatted_accept = self.accept.split(": ").collect::<Vec<&str>>()[1];
        let mut types = formatted_accept.split(";").collect::<Vec<&str>>();
        types.pop();
        let mut handled_types: Vec<(String, f32)> = Vec::new();
        for i in 1..types.len() {
            types[i] = &types[i][6..];
        }

        for i in 0..types.len() {
            let type_vec = types[i].split(",").collect::<Vec<&str>>();
            for j in 0..type_vec.len() {
                handled_types.push((type_vec[j].to_string(), handled_weidhts[i]));
            }
        }
        print!("{:?}",handled_types);


        return vec![];
    }
}
