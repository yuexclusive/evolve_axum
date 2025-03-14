# \UserApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_pwd**](UserApi.md#change_pwd) | **PUT** /v1/user/change_pwd | 
[**delete**](UserApi.md#delete) | **DELETE** /v1/user/delete | 
[**get**](UserApi.md#get) | **GET** /v1/user/{id} | 
[**register**](UserApi.md#register) | **POST** /v1/user/register | 
[**search**](UserApi.md#search) | **GET** /v1/user/search | 
[**send_email_code**](UserApi.md#send_email_code) | **POST** /v1/user/send_email_code | 
[**update**](UserApi.md#update) | **PUT** /v1/user/update | 
[**validate_exist_email**](UserApi.md#validate_exist_email) | **GET** /v1/user/validate_exist_email/{email} | 
[**validate_not_exist_email**](UserApi.md#validate_not_exist_email) | **GET** /v1/user/validate_not_exist_email/{email} | 



## change_pwd

> models::MsgResp change_pwd(change_password_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**change_password_req** | [**ChangePasswordReq**](ChangePasswordReq.md) |  | [required] |

### Return type

[**models::MsgResp**](MsgResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete

> models::MsgResp delete(user_delete_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_delete_req** | [**UserDeleteReq**](UserDeleteReq.md) |  | [required] |

### Return type

[**models::MsgResp**](MsgResp.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get

> models::UserGetResp get(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | user id | [required] |

### Return type

[**models::UserGetResp**](UserGetResp.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register

> models::MsgResp register(register_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**register_req** | [**RegisterReq**](RegisterReq.md) |  | [required] |

### Return type

[**models::MsgResp**](MsgResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search

> models::UserSearchResp search(page_index, page_size, key_word)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page_index** | **i64** |  | [required] |
**page_size** | **i64** |  | [required] |
**key_word** | Option<**String**> |  |  |

### Return type

[**models::UserSearchResp**](UserSearchResp.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_email_code

> models::SendEmailCodeResp send_email_code(send_email_code_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_email_code_req** | [**SendEmailCodeReq**](SendEmailCodeReq.md) |  | [required] |

### Return type

[**models::SendEmailCodeResp**](SendEmailCodeResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update

> models::UserUpdateResp update(user_update_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_update_req** | [**UserUpdateReq**](UserUpdateReq.md) |  | [required] |

### Return type

[**models::UserUpdateResp**](UserUpdateResp.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_exist_email

> models::MsgResp validate_exist_email(email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | email | [required] |

### Return type

[**models::MsgResp**](MsgResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_not_exist_email

> models::MsgResp validate_not_exist_email(email)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | email | [required] |

### Return type

[**models::MsgResp**](MsgResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

