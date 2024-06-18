<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Enterprise Accounts</name>
   <tag></tag>
   <elementGuidId>c769121f-6434-40ac-a107-47ce88f7ccd0</elementGuidId>
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
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>d5a466d8-bb8e-4fe1-9601-e749163c3bed</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${bearerToken}</value>
      <webElementGuid>dac720f4-c667-43a9-b088-4214e08fe079</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${baseUrl}/user/enterprise-accounts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseUrl</defaultValue>
      <description></description>
      <id>8bbe65b0-97b6-4bda-922e-925566137481</id>
      <masked>false</masked>
      <name>baseUrl</name>
   </variables>
   <variables>
      <defaultValue>'eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxNDgxIiwicmVnaXN0cmF0aW9uX2lkIjoxNDgxLCJleHAiOjE3MjIyMTM5MDcsImlhdCI6MTcxODYxMzkwNywiYXV0aG9yaXR5IjoiTUFLRVJfUk9MRSJ9.QfVBEVdSrk3g30yfqiXD_2KxZYE74aBAM60V2janvLX2tyTz-wlxeA2D_oTBmyKsUBB9odVg7slI1eflWIMG6w'</defaultValue>
      <description></description>
      <id>5a520a33-688f-45d0-a088-de9ed0b08e90</id>
      <masked>false</masked>
      <name>bearerToken</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

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
  &quot;\$schema&quot;: &quot;http://json-schema.org/draft-07/schema#&quot;,
  &quot;title&quot;: &quot;Generated schema for Root&quot;,
  &quot;type&quot;: &quot;object&quot;,
  &quot;properties&quot;: {
    &quot;enterpriseAccount&quot;: {
      &quot;type&quot;: &quot;object&quot;,
      &quot;properties&quot;: {
        &quot;id&quot;: {
            &quot;type&quot;: &quot;number&quot;
		},
        &quot;taxCode&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;companyName&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;contactPersonName&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;contactPersonPhone&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;contactPersonEmail&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;supportStaffEmail&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;openingAccountPurpose&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;consent&quot;: {
            &quot;type&quot;: &quot;boolean&quot;
		},
        &quot;createdDate&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;lastModifiedDate&quot;: {
            &quot;type&quot;: &quot;string&quot;
		},
        &quot;registrationAccount&quot;: {
            &quot;type&quot;: &quot;object&quot;,
            &quot;properties&quot;: {
                &quot;id&quot;: {
                    &quot;type&quot;: &quot;number&quot;
                },
                &quot;province&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;district&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;branch&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;accountNumber&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                },
                &quot;selectedAccountType&quot;: {
                    &quot;type&quot;: &quot;string&quot;
                }
            }
		},
        &quot;required&quot;: [
            &quot;id&quot;,
            &quot;taxCode&quot;,
            &quot;companyName&quot;,
            &quot;contactPersonName&quot;,
            &quot;contactPersonPhone&quot;,
            &quot;contactPersonEmail&quot;,
            &quot;supportStaffEmail&quot;,
            &quot;openingAccountPurpose&quot;,
            &quot;consent&quot;,
            &quot;registrationAccount&quot;
        ]
      }
		}
},
  &quot;required&quot;: [
    &quot;enterpriseAccount&quot;
  ]
}
&quot;&quot;&quot;
boolean successful = WS.validateJsonAgainstSchema(response,jsonPass)
WebUI.comment(successful.toString())
}
catch(Exception e) {
	WebUI.comment(&quot;API failed&quot;)
}</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
