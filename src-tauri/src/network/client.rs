use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ClientConnection, RootCertStore, Stream};

pub struct HttpsClient {
    host: String,
    port: u16,
    config: Arc<ClientConfig>,
}

impl HttpsClient {
    pub fn new(host: impl Into<String>, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let host = host.into();

        // Load root certificates
        let root_store = RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.into(),
        };

        // Build TLS client configuration
        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(Self {
            host,
            port,
            config: Arc::new(config),
        })
    }

    fn connect(&self) -> Result<(ClientConnection, TcpStream), Box<dyn std::error::Error>> {
        let server_name: ServerName<'static> = self.host.clone().try_into()?;
        let conn = ClientConnection::new(Arc::clone(&self.config), server_name)?;

        let sock = TcpStream::connect((self.host.as_str(), self.port))?;

        Ok((conn, sock))
    }

    /// Performs HTTP get request to specified path return response body as string
    pub fn get(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Construct a HTTP GET request
        let request = format!(
            "GET {} HTTP/1.1\r\n\
            Host: {}\r\n\
            Connection: close\r\n\
            Accept-Encoding: identity\r\n\
            \r\n",
            path, self.host
        );

        self.send_raw_request(&request)
    }

    /// Sends a raw HTTP request and returns the response as a string
    pub fn send_raw_request(
        &self,
        request: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Establish a TLS connection
        let (mut conn, mut sock) = self.connect()?;
        let mut tls = Stream::new(&mut conn, &mut sock);

        // Send the HTTP request
        tls.write_all(request.as_bytes())?;
        tls.flush()?;

        // Output the negotiated ciphersuite 
        if let Some(ciphersuite) = tls.conn.negotiated_cipher_suite() {
            eprintln!("Current ciphersuite: {:?}", ciphersuite.suite());
        }

        let mut buffer = Vec::new();
        tls.read_to_end(&mut buffer)?;

        let response = String::from_utf8(buffer)?;
        Ok(response)
    }
}