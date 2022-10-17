# User

```json
[
    "User":{
        "_id":"53102b43b..."//ID
        "EncodingUsername":"6a,6a,b6...",//String
        "EncodingPassword":"6a,6a,b6...",//String
        "RegistrationTime":1665983027940,//f64
    }
]
```

# ActiveToken

```json
[
    "ActiveToken":{
        "_id":"2b43b85310b0..."//ID
        "ActiveUser":"53102b43b..."//ID from User
        "SignatureSecret":"tja95orm7e..."//String
    }
]
```