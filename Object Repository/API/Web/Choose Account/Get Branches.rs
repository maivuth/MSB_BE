<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Branches</name>
   <tag></tag>
   <elementGuidId>14e24716-f7bc-4e54-a0d2-d386635315ab</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${bearerToken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearerToken}</value>
      <webElementGuid>05eaf721-265b-457f-b4b6-50223d070db6</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/user/locations/branches?districtCode=${districtCode}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>015861c3-9cb4-4f50-81e6-6243837c492b</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIyMTM5MDcsImlhdCI6MTcxODYxMzkwNywiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QfVBEVdSrk3g30yfqiXD_2KxZYE74aBAM60V2janvLX2tyTz-wlxeA2D_oTBmyKsUBB9odVg7slI1eflWIMG6w'</defaultValue>
      <description></description>
      <id>1ff96d19-32ec-443c-8ed0-f0213a6f43bd</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <variables>
      <defaultValue>'70131'</defaultValue>
      <description></description>
      <id>5a23d614-b424-4562-89f6-bc7246542a71</id>
      <masked>false</masked>
      <name>districtCode</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

try {
	WS.verifyResponseStatusCode(response, 200)
	
	assertThat(response.getStatusCode()).isEqualTo(200)
	
	String jsonPass =
	&quot;&quot;&quot;
{
  &quot;\$id&quot;: &quot;https://example.com/person.schema.json&quot;,
  &quot;\$schema&quot;: &quot;https://json-schema.org/draft/2020-12/schema&quot;,
  &quot;title&quot;: &quot;DistrictCodeList&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;provinceCodeList&quot;: {&quot;type&quot;: &quot;null&quot;},
    &quot;districtCodeList&quot;: {&quot;type&quot;: &quot;null&quot;},
    &quot;branchCodeList&quot;: {
      &quot;type&quot;: &quot;array&quot;,
      &quot;item&quot;: {
        &quot;additionalProperties&quot;: false,
        &quot;properties&quot; : {
            &quot;code&quot;: {&quot;type&quot;: &quot;string&quot;},
            &quot;description&quot;: {&quot;type&quot;: &quot;string&quot;}
        },
        &quot;required&quot;: [&quot;code&quot;, &quot;description&quot;]
      }
    }
  },
  &quot;required&quot;: [&quot;districtCodeList&quot;]
}
&quot;&quot;&quot;
	boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
	}
	catch (Exception e){
		throw &quot;API failed&quot;
	}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
