# Article service

This is the repository for the active development of the article micro-service maded for a webpage.
The API is an application/json service, meaning all the requests and responses are with JSON format.
You can see all the request formats and responses further bellow.

> *This repo is still on dev*



## API
You can find the documents of the API on the repository with the Open API files. We will also explain all the endpoints and requests here and how you should perform a request. You can also see the response types and what to expect.

## Generic Request and Response schema
**Request schema**

```json
{
    "content": object
}
```
The json above represents the generic JSON request for the api. The parameter `content` represents diferent types depending on the endpoint. Diferent types as an article id, size of how many articles to retrieve, an article json object...

**Response schema**

```json
{
    "code": number,
    "description": string,
    "datetime": DateTime,
    "content_size": number,
    "content": Array
}
```
The response schema contains a few parameters:

A code, representing the status of the response, the description of the status code, the datetime from the request, content size represents the size of the array fo the response content. And finally we have the content which is an array of diferent posibilities.
content can have a value of an `id` response, some `articles`, or an `error` type.

**Article schema**
```json
{
    "id": string,
    "author": string,
    "created_at": DateTime,
    "last_update": DateTime,
    "content": string
}
```
This json represents the structure of the Article

**Error schema**
```json
{
    "code": string,
    "description": string,
    "reason": string
}
```
This json represents the structure of the error type. The code represents the status of the operation, the description of the status code and the reason behind the error.

**There are a few error codes:**

|Code| Description   |
|--- |---            |
| 0  | Invalid ID    |
| 1  | Not found     |
| 2  | Malformed JSON|
| 3  | Internal      |
| 4  | DuplicateKey  |
| 5  | Timeout       |

## Endpoints
### **/api/find**

This endpoints delivers an article with the same id as given in the request.

**Request json**
```json
    {
        "content": string
    }
```
The `content` parameter of the request represents the id of the article you want to retrieve.
The server returns the response object mentioned before with 2 content posibble values, the article or the error 
### **/api/list**

This endpoints delivers an article with the same id as given in the request.

**Request json**
```json
    {
        "content": string
    }
```
The `content` parameter of the request represents the amount of articles you want to retrieve from the database.
The server returns the response object mentioned before with 2 content posibble values, an array of articles or the error of the request.

### **/api/insert**

This endpoints delivers an article with the same id as given in the request.

 **Request json**
```json
    {
        "content":
            {
            "id": string,
            "author": string,
            "created_at": DateTime,
            "last_update": DateTime,
            "content": string
            }
    }
```
The `content` parameter of the request represents the article which you want to insert in the databse. The schema to follow is the mentioned before.

This returns a response object with the article id if successful or the error 






