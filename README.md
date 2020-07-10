# actix-sample-api

This is a simple demo project designed to learn rust and actix_web
the task was to implement on Rust a simple REST API with functionality that loads images.
Images are uploaded to disk.

To run a command
`docker-compose up -d --build`

  - Ability to upload multiple files.
  - Ability to accept multipart / form-data requests.
  - Ability to accept JSON requests with BASE64 encoded
images.
  - Ability to upload images at a given URL (image
posted somewhere on the Internet).
  - Create a square preview of the image size 100px by 100px.

3 endpoints are implemented,            
plus additionally 2 endpoins using which you can see the downloaded pictures by endpoint `0.0.0.0:8088/store`                   
preview `0.0.0.0:8088/preview`
1) load_image
Upload single image               
```
curl --location --request POST '0.0.0.0:8088' \          
--header 'Content-Type: multipart / form-data' \           
--form '=@/Users/user1/Documents/images/sad-3.png'
```

You can upload several images
```
curl --location --request POST '0.0.0.0:8088' \
--header 'Content-Type: multipart / form-data' \
--form '=@/Users/user1/Documents/images/sad-3.png' \
--form '=@/Users/user1/Documents/images/sad.jpg' \
--form '=@/Users/user1/Documents/images/sad-2.jpeg' \
```

2) load_decode_image
Loading an image encoded in base64
```
curl --location --request POST '0.0.0.0:8088/load_decode_image?name=sad214.jpeg' \
--header 'Content-Type: multipart / form-data' \
--form '= / 9j / 2wCEAAgGBgcGBQgHBwcJ ....... aLjP / Z'
```
3) load_image_by_url
Download image by url from internet            
`curl --location --request POST '0.0.0.0:8088/load_by_url?url=https://www.rust-lang.org/logos/rust-logo-512x512.png'`

Some additional features:
- for each of the endpoints a preview image is created with a size of 100px by 100px
- there is a graceful shutdown - it is out of the box at actix_web
- a docker image for the project was created, a multi-stage image creation was used, build on the rust image                 
Then it was copied to the `image gcr.io/distroless/cc-debian10` - the small(35 mb.) and secure image from google.       
- travis ci, gitlab ci, docker compose
- tests were written for part of the functional, for functional with loading multipart image is not implemented.
