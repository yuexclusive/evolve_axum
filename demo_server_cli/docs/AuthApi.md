# \AuthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authorize**](AuthApi.md#authorize) | **POST** /v1/auth/authorize | 
[**user_info**](AuthApi.md#user_info) | **GET** /v1/auth/user_info | 



## authorize

> models::AuthResp authorize(auth_req)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_req** | [**AuthReq**](AuthReq.md) |  | [required] |

### Return type

[**models::AuthResp**](AuthResp.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_info

> models::UserInfoResp user_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserInfoResp**](UserInfoResp.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

