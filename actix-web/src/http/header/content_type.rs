use mime::Mime;

use super::CONTENT_TYPE;

crate::http::header::common_header! {
    /// `Content-Type` header, defined
    /// in [RFC 7231 §3.1.1.5](https://datatracker.ietf.org/doc/html/rfc7231#section-3.1.1.5)
    ///
    /// The `Content-Type` header field indicates the media type of the
    /// associated representation: either the representation enclosed in the
    /// message payload or the selected representation, as determined by the
    /// message semantics.  The indicated media type defines both the data
    /// format and how that data is intended to be processed by a recipient,
    /// within the scope of the received message semantics, after any content
    /// codings indicated by Content-Encoding are decoded.
    ///
    /// Although the `mime` crate allows the mime options to be any slice, this crate
    /// forces the use of Vec. This is to make sure the same header can't have more than 1 type. If
    /// this is an issue, it's possible to implement `Header` on a custom struct.
    ///
    /// # ABNF
    /// ```plain
    /// Content-Type = media-type
    /// ```
    ///
    /// # Example Values
    /// * `text/html; charset=utf-8`
    /// * `application/json`
    ///
    /// # Examples
    /// ```
    /// use actix_web::HttpResponse;
    /// use actix_web::http::header::ContentType;
    ///
    /// let mut builder = HttpResponse::Ok();
    /// builder.insert_header(
    ///     ContentType::json()
    /// );
    /// ```
    ///
    /// ```
    /// use actix_web::HttpResponse;
    /// use actix_web::http::header::ContentType;
    ///
    /// let mut builder = HttpResponse::Ok();
    /// builder.insert_header(
    ///     ContentType(mime::TEXT_HTML)
    /// );
    /// ```
    (ContentType, CONTENT_TYPE) => [Mime]

    test_parse_and_format {
        crate::http::header::common_header_test!(
            test1,
            vec![b"text/html"],
            Some(HeaderField(mime::TEXT_HTML)));
    }
}

impl ContentType {
    /// A constructor to easily create a `Content-Type: application/json`
    /// header.
    #[inline]
    pub fn json() -> ContentType {
        ContentType(mime::APPLICATION_JSON)
    }

    /// A constructor to easily create a `Content-Type: text/plain;
    /// charset=utf-8` header.
    #[inline]
    pub fn plaintext() -> ContentType {
        ContentType(mime::TEXT_PLAIN_UTF_8)
    }

    /// A constructor to easily create a `Content-Type: text/html; charset=utf-8`
    /// header.
    #[inline]
    pub fn html() -> ContentType {
        ContentType(mime::TEXT_HTML_UTF_8)
    }

    /// A constructor to easily create a `Content-Type: text/xml` header.
    #[inline]
    pub fn xml() -> ContentType {
        ContentType(mime::TEXT_XML)
    }

    /// A constructor to easily create a `Content-Type:
    /// application/www-form-url-encoded` header.
    #[inline]
    pub fn form_url_encoded() -> ContentType {
        ContentType(mime::APPLICATION_WWW_FORM_URLENCODED)
    }

    /// A constructor to easily create a `Content-Type: image/jpeg` header.
    #[inline]
    pub fn jpeg() -> ContentType {
        ContentType(mime::IMAGE_JPEG)
    }

    /// A constructor to easily create a `Content-Type: image/png` header.
    #[inline]
    pub fn png() -> ContentType {
        ContentType(mime::IMAGE_PNG)
    }

    /// A constructor to easily create a `Content-Type:
    /// application/octet-stream` header.
    #[inline]
    pub fn octet_stream() -> ContentType {
        ContentType(mime::APPLICATION_OCTET_STREAM)
    }
}
