<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_.stepper__navigation_assis_1</name>
   <tag></tag>
   <elementGuidId>a8070cbd-d276-48a5-a50e-25c4f90a6a90</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>stepper js-stepper is-active</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    


    .stepper__navigation_assistance {
        max-width: 870px;
        justify-content: unset;
    }

        .stepper__navigation_assistance li:first-child {
            margin-right: 100px;
        }




    

                
                    
                        1
                        General Information
                    
                
                
                    
                        2
                        Scope of Service
                    
                
                
                    
                        3
                        Shipment information
                    
                
                
                    
                        4
                        Additional Information
                    
                
    
        
            
                * Mandatory fields
                
                
                    
                        
                            ej.createObject(&quot;ej.dataSources.PricingMethod&quot;,{&quot;data0&quot;:ej.isJSON([{&quot;Value&quot;:&quot;Flat&quot;,&quot;Key&quot;:&quot;1&quot;,&quot;ObjectId&quot;:0},{&quot;Value&quot;:&quot;Per 100 lbs&quot;,&quot;Key&quot;:&quot;4&quot;,&quot;ObjectId&quot;:0}])});
                            Pricing Method *
                            

                        
                    
                    
                        
                            
                            Agent Reference Number
                        
                    
                

                
                    
                        
                             Is it for a RMC? *

                            
                                
                                    
                                    Yes
                                
                                
                                    
                                    No
                                
                            
                            
                        
                    
                

                
                    
                        
                            
                            Comments
                        
                    
                

                
    .error {
        color: #d50000;
        position: absolute;
        font-size: 12px;
        margin-top: 3px;
        display: block;
    }

    .font {
        font-size: 10px;
        color: dodgerblue;
    }

    div.static {
        position: relative;
        left: 240px;
    }
    .mdl-button.mdl-js-button.mdl-button--icon.delete-btn.btn-remove-row {
        display: flex;
        justify-content: center;
        align-items: center;
    }

Documents
Attached any other document necessary concerning the request. 

    
        
            Add File or drop here
        
    
    
        
            
                
                    
                        
                    
                
                
                    
                        
                    
                
            
        
    







    
Insurance
Proof of existance

This field cannot be empty

    var currentselections = null;
    $(document).ready($(function (e) {
        //Retrieving text to be displayed
        CreateUploadBoxWithText(&quot;dropAreaText&quot;, $('#LeadId').val() );
        GetUploadedDoc($('#LeadId').val());
    }));

   
    function CreateUploadBoxWithText(strKey, iAgentLeadId) {
        $.ajax({
            url: &quot;/UploadBox/GetDisplayText?qid=&quot; + $('body').data('usid'),
            data: { &quot;strKey&quot;: strKey },
            method: &quot;GET&quot;,
            timeout:5000,
            success: function (data) {
                if (data.length !== 0) {
                    CreateUploadBox(data.toString(), iAgentLeadId);
                }
            },
            error: function (jqXHR, exception) {
                HandleAjaxError(jqXHR);
            }
        });
    }
    function CreateUploadBox(strText, iAgentLeadId) {
        $(&quot;#Uploadbox&quot;).ejUploadbox({
            saveUrl: &quot;/UploadBox/Save?qid=&quot; + $('body').data('usid') + &quot;&amp;aid=&quot; + iAgentLeadId,
            allowDragAndDrop: true,
            multipleFilesSelection: false,
            dropAreaHeight: &quot;50px&quot;,
            dropAreaWidth: &quot;800px&quot;,
            buttonText: { browse: &quot;&quot; },
            // asyncUpload: false,
            dropAreaText: strText,
            showBrowseButton: false,
            showFileDetails: false,
            fileSelect: &quot;LaunchUpload&quot;,
            complete: &quot;UploadedFiles&quot;,
            // error:&quot;DisplayUploadError&quot;
            autoUpload: true,
        });
    }
    function DisplayUploadError(args) {
        $(&quot;#Error&quot;).text(&quot;File name:&quot; + args.files.name + &quot; &quot; + args.error);
        $(&quot;#Error&quot;).addClass(&quot;error&quot;);
    }
    function LaunchUpload(args) {
        var strFileExt = &quot;&quot;;
        var count = 0;
        $(&quot;#Error&quot;).removeClass(&quot;error&quot;);
        $(&quot;#Error&quot;).empty();
        $(args.files).each(function () {
            ++count;
        });

        if (count > 1) {
            $.ajax({
                url: &quot;/UploadBox/GetDisplayText?qid=&quot; + $('body').data('usid'),
                data: { &quot;strKey&quot;: &quot;UploadSingleFile&quot; },
                method: &quot;GET&quot;,
                timeout: 10000,
                success: function (data) {
                    if (data.length !== 0) {
                        $(&quot;#Error&quot;).html(data.toString());
                        $(&quot;#Error&quot;).addClass(&quot;error&quot;);
                    }
                    else {
                        $(&quot;#Error&quot;).html(&quot;Choose only one file to upload at a time.&quot;);
                        $(&quot;#Error&quot;).addClass(&quot;error&quot;);
                    }
                },
                error: function (jqXHR, exception) {
                    HandleAjaxError(jqXHR);
                }
            });


            return false;
        }
        if (args.files[0] != null) {
            if (args.files[0].size > parseInt('20971520')) {
                $(&quot;#Error&quot;).html('The size of the file uploaded cannot exceed 20 MB.');
                $(&quot;#Error&quot;).addClass(&quot;error&quot;);
            }
        }
    }
    function RefreshUploadPreview(data) {
        $(&quot;#uploaded&quot;).empty();
        var strPreviewHtml = '&lt;table id=&quot;UploadedDoc&quot;>&lt;tr>&lt;td>&lt;/td>&lt;td id=&quot;DocTypeTitle&quot;>&lt;/td>&lt;td>&lt;/td>&lt;/tr>';
        for (var i = 0; i &lt; data.length; i++) {
            strPreviewHtml = strPreviewHtml + '&lt;tr>&lt;td  style=&quot;width: 30%;&quot; class=&quot;font&quot;>' + data[i].DocOriginalName + '&lt;/td>&lt;td id=&quot;' + data[i].DocId + '&quot;> &lt;/td>&lt;td  style=&quot;width: 5%;&quot;>&lt;div onclick=RemoveFile(&quot;' + data[i].DocId + '&quot;); class=&quot;mdl-button mdl-js-button mdl-button--icon delete-btn btn-remove-row&quot; data-upgraded=&quot;,MaterialButton&quot;>&lt;i class=&quot;icon-delete&quot;>&lt;/i >&lt;/div >&lt;/td>&lt;td style=&quot;width: 65%;&quot;>&lt;/td>';
        }
        strPreviewHtml = strPreviewHtml + '&lt;/table>';
        $(&quot;#uploaded&quot;).append(strPreviewHtml);
        var uploadObj = $(&quot;#Uploadbox&quot;).data(&quot;ejUploadbox&quot;);
        uploadObj.refresh();

    }
    var DocTypeDict = new Object();  

    function LoadDropDownListinTable(data)
    {
        var NewDropDownlist;
        var NewErrorMsg;
        if (data.length != 0)
        {
            $(&quot;#UploadedDoc&quot;).find('td[id^=&quot;' + &quot;DocTypeTitle&quot; + '&quot;]').html(&quot;Document Type&quot; + &quot; *&quot;);
            $(&quot;#UploadedDoc&quot;).find('td[id^=&quot;' + &quot;DocTypeTitle&quot; + '&quot;]').attr('style', 'color:rgb(154, 154, 154);display:inline-block;font-family:robotoregular, Arial, Helvetica, sans-serif;font-size:13px;font-weight:400;vertical-align: middle;padding:0px;margin-left:15px;');
        }
        for (var i = 0; i &lt; data.length; i++) {
            NewDropDownlist = $(&quot;#DocType&quot;).clone();
            NewDropDownlist.attr('id',  data[i].DocId);
            NewDropDownlist.attr('name', data[i].DocId);
            NewDropDownlist.attr('data-selected', data[i].DocType);
            NewDropDownlist.addClass('DocType');
            NewDropDownlist.css('visibility', 'visible');
            $(&quot;#UploadedDoc&quot;).find('td[id^=&quot;' + data[i].DocId + '&quot;]').html(NewDropDownlist);            
            //Mapping DocId and dropdown list option
            DocTypeDict[data[i].DocId] = $(&quot;#&quot; + data[i].DocId).val();
            //Error message
            NewErrorMsg = $(&quot;#ErrorMsg&quot;).clone();
            NewErrorMsg.attr('id', data[i].DocId);
            NewErrorMsg.addClass(&quot;fileType__error-message&quot;);
            $(&quot;#UploadedDoc&quot;).find('td[id^=&quot;' + data[i].DocId + '&quot;]').css('vertical-align','top');
            $(&quot;#UploadedDoc&quot;).find('td[id^=&quot;' + data[i].DocId + '&quot;]').append('&lt;div style=&quot;visibility: hidden&quot; id=&quot;' + data[i].DocId+'&quot;>');
            $(&quot;#UploadedDoc&quot;).find('td[id^=&quot;' + data[i].DocId + '&quot;]').append(NewErrorMsg);
            $(&quot;#UploadedDoc&quot;).find('td[id^=&quot;' + data[i].DocId + '&quot;]').append('&lt;/div>');
            // load selected option.
            $(&quot;#&quot; + data[i].DocId).find(&quot;option&quot;).filter(function () {
                return ($(this).val() == data[i].DocType);
            }).prop('selected', true);

        }
        $(&quot;.DocType&quot;).each(function (i, element) {
            $(element).ejDropDownList({
                enableFilterSearch: true,
                change: function (args) {
                    SaveDocType($(this.element).attr(&quot;id&quot;).split('_')[0] + &quot;:&quot; + args.value);
                    $(this.element).closest(&quot;td&quot;).find(&quot;.fileType__error-message&quot;).hide();
                },
                value: $(element).data(&quot;selected&quot;),
                cssClass: &quot;required&quot;
            });
        });
    }
    function UploadedFiles(args) {
        if (args.responseText.trim().length &lt; 1) {
            $(&quot;#Error&quot;).removeClass(&quot;error&quot;);
            $(&quot;#Error&quot;).empty();
            $(args.files).each(function () {
                // Calling refresh DisplayPreview
                GetUploadedDoc($('#LeadId').val());
            });
        }
        else {
            $(&quot;#Error&quot;).html(args.responseText);
            $(&quot;#Error&quot;).addClass(&quot;error&quot;);
        }
    }

    function SaveDocType(DocType) {
        $.ajax({
            url: &quot;/UploadBox/SaveDocType?qid=&quot; + $('body').data('usid'),
            data: { &quot;strDocType&quot;: DocType },
            method: &quot;POST&quot;,            
            error: function (jqXHR, exception) {
                HandleAjaxError(jqXHR);
            }
        })
    }
    function GetUploadedDoc(lAgentLeadID) {
        $.ajax({
            url: &quot;/UploadBox/DisplayFiles?qid=&quot; + $('body').data('usid'),
            data: { &quot;strAgentLeadId&quot;: lAgentLeadID },
            method: &quot;POST&quot;,
            success: function (data) {
                if (data.length !== 0) {
                    RefreshUploadPreview(data, $('#LeadId').val());
                    LoadDropDownListinTable(data);
                }
            },
            error: function (jqXHR, exception) {
                HandleAjaxError(jqXHR);
            }
        })
    }

    function RemoveFile(strId) {
        $.ajax({
            url: &quot;/UploadBox/RemoveFile?qid=&quot; + $('body').data('usid'),
            data: { &quot;strId&quot;: strId, &quot;aid&quot;: $('#LeadId').val() },
            method: &quot;POST&quot;,
            success: function (data) {
                RefreshUploadPreview(data, strId);
                LoadDropDownListinTable(data);
            },
            error: function (jqXHR, exception) {
                HandleAjaxError(jqXHR);
            }
        });
    }
    function HandleAjaxError(jqXHR) {
        var msg = '';
        if (jqXHR.status === 0) {
            msg = 'Not connect.\n Verify Network.';
        } else if (jqXHR.status == 404) {
            msg = 'Requested page not found. [404]';
        } else if (jqXHR.status == 500) {
            msg = 'Internal Server Error [500].';
        } else if (exception === 'parsererror') {
            msg = 'Requested JSON parse failed.';
        } else if (exception === 'timeout') {
            msg = 'Time out error.';
        } else if (exception === 'abort') {
            msg = 'Ajax request aborted.';
        } else {
            msg = 'Uncaught Error.\n' + jqXHR.responseText;
        }
        $('#Error').html(msg);
    }
    
    //Make validation error invisible on focus
    function MakeErrorMsgHidden(strControlName) {
        $(&quot;#&quot; + strControlName + &quot;.fileType__error-message&quot;).css(&quot;visibility&quot;, &quot;hidden&quot;);
    }




$(function($){$(&quot;#PricingMethod&quot;).ejDropDownList({&quot;enableFilterSearch&quot;:true,&quot;dataSource&quot;:ej.isJSON([{&quot;Value&quot;:&quot;Flat&quot;,&quot;Key&quot;:&quot;1&quot;,&quot;ObjectId&quot;:0},{&quot;Value&quot;:&quot;Per 100 lbs&quot;,&quot;Key&quot;:&quot;4&quot;,&quot;ObjectId&quot;:0}]),&quot;fields&quot;:{&quot;text&quot;:&quot;Value&quot;,&quot;value&quot;:&quot;Key&quot;},&quot;cssClass&quot;:&quot;required&quot;,&quot;value&quot;:&quot;3&quot;,&quot;locale&quot;:&quot;en-GB&quot;,&quot;change&quot;:&quot;removeError&quot;});});

                
                    
                        
                            Previous
                            Shipment Information
                        

                        
                    

                    
                        
                            Submit request
                        

                        
                    
                
            
        

//&lt;![CDATA[
if (!window.mvcClientValidationMetadata) { window.mvcClientValidationMetadata = []; }
window.mvcClientValidationMetadata.push({&quot;Fields&quot;:[{&quot;FieldName&quot;:&quot;PricingMethod&quot;,&quot;ReplaceValidationMessageContents&quot;:true,&quot;ValidationMessageId&quot;:&quot;PricingMethod_validationMessage&quot;,&quot;ValidationRules&quot;:[{&quot;ErrorMessage&quot;:&quot;The field PricingMethod must be a number.&quot;,&quot;ValidationParameters&quot;:{},&quot;ValidationType&quot;:&quot;number&quot;}]},{&quot;FieldName&quot;:&quot;RMC&quot;,&quot;ReplaceValidationMessageContents&quot;:true,&quot;ValidationMessageId&quot;:&quot;RMC_validationMessage&quot;,&quot;ValidationRules&quot;:[]}],&quot;FormId&quot;:&quot;form0&quot;,&quot;ReplaceValidationSummary&quot;:false});
//]]>
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;AjaxBody&quot;)/div[@class=&quot;stepper js-stepper is-active&quot;]</value>
   </webElementProperties>
</WebElementEntity>
