<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>clickTable</name>
   <tag></tag>
   <elementGuidId>7efab893-5b63-4e29-8396-9b6266a5cf95</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Today is'])[1]/following::center[1]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>center</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
        
            
                




#headerTable
{
	background: #015294;
	background-image:url('/img/niaidglobaljavascripts/images/nihHeaderImgWithTxt.jpg');
	background-repeat:no-repeat;
	background-position:left center; 
	font-size: 22px;
    	font-family: sans-serif;    	
	color: White;	
	text-align: center;
	font-weight: bold;	
	height: 75px;  
} 
.displayName
{
	font-size:12px;
}

 input:disabled
  {
      background-color:ghostWhite;
  }


    
	 
	DAIT - ORA - Clinical Trial Agreements
	Shalini Tahiliani
	 
    


            
			
            
            
                
                    
                        QA - Generate Agreement
                    
                
            
            	
			
			
			
                
				
				
				  
				    
				
				
				
					








    
        
            
                
                    Instructions 
                
                
                    
                        
                            
				                
Select the action type (new/amend)
Complete  the fields below and click Generate Agreement. Required fields are marked with an *
The Agreement document(s) will be automatically named with the Agreement Number - Company Name  
You will be notified via e-mail within the next few minutes of  Agreement generation 
   
                            
                        
                    
                
            
        
    

					
					




                  
               
                
              
                 
                 
              
                 
           
              










    
    var agreementTypeArr = {&quot;actualRows&quot;:8,&quot;filteredRows&quot;:8,&quot;myRows&quot;:[{&quot;AgreementType&quot;:&quot;Clinical Trial Agreement&quot;,&quot;AgreementTypePrefix&quot;:&quot;T&quot;},{&quot;AgreementType&quot;:&quot;Confidentiality Agreement&quot;,&quot;AgreementTypePrefix&quot;:&quot;D&quot;},{&quot;AgreementType&quot;:&quot;Evaluation Agreement&quot;,&quot;AgreementTypePrefix&quot;:&quot;E&quot;},{&quot;AgreementType&quot;:&quot;Letter of Agreement&quot;,&quot;AgreementTypePrefix&quot;:&quot;L&quot;},{&quot;AgreementType&quot;:&quot;Material Transfer Agreement&quot;,&quot;AgreementTypePrefix&quot;:&quot;M&quot;},{&quot;AgreementType&quot;:&quot;Memorandum of Understanding&quot;,&quot;AgreementTypePrefix&quot;:&quot;MO&quot;},{&quot;AgreementType&quot;:&quot;Non-Clinical Study&quot;,&quot;AgreementTypePrefix&quot;:&quot;N&quot;},{&quot;AgreementType&quot;:&quot;Quality Agreement&quot;,&quot;AgreementTypePrefix&quot;:&quot;Q&quot;}],&quot;sourceID&quot;:79982128,&quot;success&quot;:true,&quot;totalRows&quot;:8,&quot;totalSourceRows&quot;:8}

    var agreementTypeArray = agreementTypeArr.myRows;    



    
    var companyArr = {&quot;actualRows&quot;:227,&quot;filteredRows&quot;:227,&quot;myRows&quot;:[{&quot;CompanyName&quot;:&quot;20 \/ 20 Genesystems&quot;,&quot;CompanyCode&quot;:82},{&quot;CompanyName&quot;:&quot;3SBio&quot;,&quot;CompanyCode&quot;:67},{&quot;CompanyName&quot;:&quot;ABT Holding Company&quot;,&quot;CompanyCode&quot;:199},{&quot;CompanyName&quot;:&quot;Achaogen&quot;,&quot;CompanyCode&quot;:235},{&quot;CompanyName&quot;:&quot;Aeolus Pharmaceuticals, Inc.&quot;,&quot;CompanyCode&quot;:114},{&quot;CompanyName&quot;:&quot;Aimmune Therapeutics&quot;,&quot;CompanyCode&quot;:231},{&quot;CompanyName&quot;:&quot;Alentic Microscience, Inc&quot;,&quot;CompanyCode&quot;:196},{&quot;CompanyName&quot;:&quot;Alexion Pharmaceuticals&quot;,&quot;CompanyCode&quot;:65},{&quot;CompanyName&quot;:&quot;ALK-Abello A\/S&quot;,&quot;CompanyCode&quot;:18},{&quot;CompanyName&quot;:&quot;Allergan, Plc&quot;,&quot;CompanyCode&quot;:214},{&quot;CompanyName&quot;:&quot;Allertein&quot;,&quot;CompanyCode&quot;:63},{&quot;CompanyName&quot;:&quot;Amgen&quot;,&quot;CompanyCode&quot;:27},{&quot;CompanyName&quot;:&quot;Angion, Inc.&quot;,&quot;CompanyCode&quot;:171},{&quot;CompanyName&quot;:&quot;Araim Pharmaceuticals, Inc.&quot;,&quot;CompanyCode&quot;:72},{&quot;CompanyName&quot;:&quot;Arca Bioharma, Inc&quot;,&quot;CompanyCode&quot;:130},{&quot;CompanyName&quot;:&quot;Armed Forces Radiobiology Research Institute (AFRRI)&quot;,&quot;CompanyCode&quot;:186},{&quot;CompanyName&quot;:&quot;Aronora, Inc.&quot;,&quot;CompanyCode&quot;:232},{&quot;CompanyName&quot;:&quot;ArQule&quot;,&quot;CompanyCode&quot;:38},{&quot;CompanyName&quot;:&quot;Astellas Pharma Global Development, Inc&quot;,&quot;CompanyCode&quot;:118},{&quot;CompanyName&quot;:&quot;Atox Bio&quot;,&quot;CompanyCode&quot;:219},{&quot;CompanyName&quot;:&quot;aTyr Pharma&quot;,&quot;CompanyCode&quot;:150},{&quot;CompanyName&quot;:&quot;Augmentative BioTherapeutics LLC&quot;,&quot;CompanyCode&quot;:230},{&quot;CompanyName&quot;:&quot;Avaxia Biologics, Inc.&quot;,&quot;CompanyCode&quot;:122},{&quot;CompanyName&quot;:&quot;Baxter Healthcare Corporation&quot;,&quot;CompanyCode&quot;:96},{&quot;CompanyName&quot;:&quot;Bayer&quot;,&quot;CompanyCode&quot;:1},{&quot;CompanyName&quot;:&quot;BCN Biosciences&quot;,&quot;CompanyCode&quot;:136},{&quot;CompanyName&quot;:&quot;Berlex&quot;,&quot;CompanyCode&quot;:2},{&quot;CompanyName&quot;:&quot;BioAegis Therapeutics&quot;,&quot;CompanyCode&quot;:247},{&quot;CompanyName&quot;:&quot;Biogen&quot;,&quot;CompanyCode&quot;:3},{&quot;CompanyName&quot;:&quot;BioIncept, LLC&quot;,&quot;CompanyCode&quot;:184},{&quot;CompanyName&quot;:&quot;Biological Mimetics, Inc.&quot;,&quot;CompanyCode&quot;:242},{&quot;CompanyName&quot;:&quot;BioMimetix&quot;,&quot;CompanyCode&quot;:249},{&quot;CompanyName&quot;:&quot;Biosyn Corporation&quot;,&quot;CompanyCode&quot;:145},{&quot;CompanyName&quot;:&quot;Bio-Tech Pharmacal&quot;,&quot;CompanyCode&quot;:69},{&quot;CompanyName&quot;:&quot;Biothera&quot;,&quot;CompanyCode&quot;:36},{&quot;CompanyName&quot;:&quot;BioTransplant Inc.&quot;,&quot;CompanyCode&quot;:4},{&quot;CompanyName&quot;:&quot;Bolder BioTechnology, Inc.&quot;,&quot;CompanyCode&quot;:28},{&quot;CompanyName&quot;:&quot;Bristol Myers Squibb&quot;,&quot;CompanyCode&quot;:19},{&quot;CompanyName&quot;:&quot;C Sixty Inc.&quot;,&quot;CompanyCode&quot;:22},{&quot;CompanyName&quot;:&quot;Calibr&quot;,&quot;CompanyCode&quot;:248},{&quot;CompanyName&quot;:&quot;California Institute for Biomedical Research&quot;,&quot;CompanyCode&quot;:241},{&quot;CompanyName&quot;:&quot;Calista Therapeutics, Inc.&quot;,&quot;CompanyCode&quot;:170},{&quot;CompanyName&quot;:&quot;Canadian Institutes of Health Research (CIHR)&quot;,&quot;CompanyCode&quot;:97},{&quot;CompanyName&quot;:&quot;Cangene&quot;,&quot;CompanyCode&quot;:73},{&quot;CompanyName&quot;:&quot;Celldex Therapeutics&quot;,&quot;CompanyCode&quot;:139},{&quot;CompanyName&quot;:&quot;Cellerant Therapeutics&quot;,&quot;CompanyCode&quot;:100},{&quot;CompanyName&quot;:&quot;Cellular Dynamics, Inc.&quot;,&quot;CompanyCode&quot;:188},{&quot;CompanyName&quot;:&quot;Centocor, Inc.&quot;,&quot;CompanyCode&quot;:7},{&quot;CompanyName&quot;:&quot;Ceramedix Holding, LLC&quot;,&quot;CompanyCode&quot;:212},{&quot;CompanyName&quot;:&quot;Ceramide Therapeutics, LLC&quot;,&quot;CompanyCode&quot;:203},{&quot;CompanyName&quot;:&quot;Chiron&quot;,&quot;CompanyCode&quot;:8},{&quot;CompanyName&quot;:&quot;Chrysalis Biotherapeutics&quot;,&quot;CompanyCode&quot;:191},{&quot;CompanyName&quot;:&quot;Circassia Limited&quot;,&quot;CompanyCode&quot;:79},{&quot;CompanyName&quot;:&quot;Cleveland Biolabs&quot;,&quot;CompanyCode&quot;:26},{&quot;CompanyName&quot;:&quot;Columbia University&quot;,&quot;CompanyCode&quot;:182},{&quot;CompanyName&quot;:&quot;Corbus Pharmaceuticals, Inc.&quot;,&quot;CompanyCode&quot;:207},{&quot;CompanyName&quot;:&quot;Coronado Biosciences&quot;,&quot;CompanyCode&quot;:154},{&quot;CompanyName&quot;:&quot;CTI Science, Inc&quot;,&quot;CompanyCode&quot;:140},{&quot;CompanyName&quot;:&quot;CuraGen Corporation&quot;,&quot;CompanyCode&quot;:60},{&quot;CompanyName&quot;:&quot;Cureveda&quot;,&quot;CompanyCode&quot;:157},{&quot;CompanyName&quot;:&quot;Cytori Therapeutics&quot;,&quot;CompanyCode&quot;:152},{&quot;CompanyName&quot;:&quot;CytoSorbents, Inc.&quot;,&quot;CompanyCode&quot;:185},{&quot;CompanyName&quot;:&quot;DBV Technologies&quot;,&quot;CompanyCode&quot;:146},{&quot;CompanyName&quot;:&quot;Dey&quot;,&quot;CompanyCode&quot;:59},{&quot;CompanyName&quot;:&quot;DiaCarta&quot;,&quot;CompanyCode&quot;:155},{&quot;CompanyName&quot;:&quot;DiaMedica&quot;,&quot;CompanyCode&quot;:135},{&quot;CompanyName&quot;:&quot;DOR BioPharma&quot;,&quot;CompanyCode&quot;:116},{&quot;CompanyName&quot;:&quot;Dova Pharmaceuticals&quot;,&quot;CompanyCode&quot;:229},{&quot;CompanyName&quot;:&quot;Dow Chemical Co.&quot;,&quot;CompanyCode&quot;:80},{&quot;CompanyName&quot;:&quot;Duke University Medical Center&quot;,&quot;CompanyCode&quot;:156},{&quot;CompanyName&quot;:&quot;Dynavax&quot;,&quot;CompanyCode&quot;:9},{&quot;CompanyName&quot;:&quot;Ed Laboratories, Inc.&quot;,&quot;CompanyCode&quot;:201},{&quot;CompanyName&quot;:&quot;Edison Pharma&quot;,&quot;CompanyCode&quot;:189},{&quot;CompanyName&quot;:&quot;Eli Lilly and Company&quot;,&quot;CompanyCode&quot;:124},{&quot;CompanyName&quot;:&quot;Eminent&quot;,&quot;CompanyCode&quot;:134},{&quot;CompanyName&quot;:&quot;Enterade, LLC&quot;,&quot;CompanyCode&quot;:167},{&quot;CompanyName&quot;:&quot;Entrinsic Health Solutions&quot;,&quot;CompanyCode&quot;:243},{&quot;CompanyName&quot;:&quot;Epistem&quot;,&quot;CompanyCode&quot;:25},{&quot;CompanyName&quot;:&quot;Epitek, Inc.&quot;,&quot;CompanyCode&quot;:147},{&quot;CompanyName&quot;:&quot;Eukarion, Inc&quot;,&quot;CompanyCode&quot;:81},{&quot;CompanyName&quot;:&quot;Exponential Biotherapies, Inc&quot;,&quot;CompanyCode&quot;:57},{&quot;CompanyName&quot;:&quot;Fibrogen&quot;,&quot;CompanyCode&quot;:41},{&quot;CompanyName&quot;:&quot;FirstString Research, Inc.&quot;,&quot;CompanyCode&quot;:220},{&quot;CompanyName&quot;:&quot;Foresee Pharmaceuticals&quot;,&quot;CompanyCode&quot;:244},{&quot;CompanyName&quot;:&quot;Fujisawa&quot;,&quot;CompanyCode&quot;:10},{&quot;CompanyName&quot;:&quot;Full Spectrum Omega&quot;,&quot;CompanyCode&quot;:224},{&quot;CompanyName&quot;:&quot;G1 Therapeutics&quot;,&quot;CompanyCode&quot;:183},{&quot;CompanyName&quot;:&quot;Genentech&quot;,&quot;CompanyCode&quot;:12},{&quot;CompanyName&quot;:&quot;Genzyme&quot;,&quot;CompanyCode&quot;:56},{&quot;CompanyName&quot;:&quot;Georgetown University&quot;,&quot;CompanyCode&quot;:194},{&quot;CompanyName&quot;:&quot;Glaxo\/Smithkline&quot;,&quot;CompanyCode&quot;:16},{&quot;CompanyName&quot;:&quot;Greer Laboratories&quot;,&quot;CompanyCode&quot;:45},{&quot;CompanyName&quot;:&quot;HA Cell Technology&quot;,&quot;CompanyCode&quot;:33},{&quot;CompanyName&quot;:&quot;Hemostem&quot;,&quot;CompanyCode&quot;:105},{&quot;CompanyName&quot;:&quot;Hollis Eden&quot;,&quot;CompanyCode&quot;:43},{&quot;CompanyName&quot;:&quot;Human Ghrelin&quot;,&quot;CompanyCode&quot;:234},{&quot;CompanyName&quot;:&quot;Humanetics Corporation&quot;,&quot;CompanyCode&quot;:137},{&quot;CompanyName&quot;:&quot;Icahn School of Medicine at Mount Sinai&quot;,&quot;CompanyCode&quot;:238},{&quot;CompanyName&quot;:&quot;IDEC Pharmaceuticals&quot;,&quot;CompanyCode&quot;:83},{&quot;CompanyName&quot;:&quot;Ilex&quot;,&quot;CompanyCode&quot;:13},{&quot;CompanyName&quot;:&quot;Immune Design&quot;,&quot;CompanyCode&quot;:193},{&quot;CompanyName&quot;:&quot;ImmuneRegen&quot;,&quot;CompanyCode&quot;:42},{&quot;CompanyName&quot;:&quot;Immunex Corporation&quot;,&quot;CompanyCode&quot;:98},{&quot;CompanyName&quot;:&quot;Inflamax&quot;,&quot;CompanyCode&quot;:163},{&quot;CompanyName&quot;:&quot;Innate Immunotherapeutics Limited&quot;,&quot;CompanyCode&quot;:210},{&quot;CompanyName&quot;:&quot;Innovation Pathways&quot;,&quot;CompanyCode&quot;:204},{&quot;CompanyName&quot;:&quot;Inverseon&quot;,&quot;CompanyCode&quot;:151},{&quot;CompanyName&quot;:&quot;Invion&quot;,&quot;CompanyCode&quot;:165},{&quot;CompanyName&quot;:&quot;InvivoSciences, Inc&quot;,&quot;CompanyCode&quot;:169},{&quot;CompanyName&quot;:&quot;Janssen Research &amp; Development, LLC&quot;,&quot;CompanyCode&quot;:195},{&quot;CompanyName&quot;:&quot;JB Therapeutics&quot;,&quot;CompanyCode&quot;:131},{&quot;CompanyName&quot;:&quot;Johnson &amp; Johnson&quot;,&quot;CompanyCode&quot;:62},{&quot;CompanyName&quot;:&quot;Kyowa Hakko Kirin Co., Ltd.&quot;,&quot;CompanyCode&quot;:132},{&quot;CompanyName&quot;:&quot;KYOWA KIRIN PHARMACEUTICAL DEVELOPMENT, INC&quot;,&quot;CompanyCode&quot;:237},{&quot;CompanyName&quot;:&quot;Lawrence Berkeley National Laboratory&quot;,&quot;CompanyCode&quot;:123},{&quot;CompanyName&quot;:&quot;LignaMed&quot;,&quot;CompanyCode&quot;:178},{&quot;CompanyName&quot;:&quot;Longevica Pharmaceuticals Inc.&quot;,&quot;CompanyCode&quot;:202},{&quot;CompanyName&quot;:&quot;MacroGenics, Inc&quot;,&quot;CompanyCode&quot;:32},{&quot;CompanyName&quot;:&quot;Maxygen, Inc&quot;,&quot;CompanyCode&quot;:61},{&quot;CompanyName&quot;:&quot;Medestea Biotech&quot;,&quot;CompanyCode&quot;:121},{&quot;CompanyName&quot;:&quot;Medical College of Wisconsin&quot;,&quot;CompanyCode&quot;:245},{&quot;CompanyName&quot;:&quot;Medical University of South Carolina&quot;,&quot;CompanyCode&quot;:222},{&quot;CompanyName&quot;:&quot;MedImmune&quot;,&quot;CompanyCode&quot;:90},{&quot;CompanyName&quot;:&quot;Meditor Pharmaceuticals, Ltd.&quot;,&quot;CompanyCode&quot;:55},{&quot;CompanyName&quot;:&quot;Medtronic MiniMed&quot;,&quot;CompanyCode&quot;:84},{&quot;CompanyName&quot;:&quot;Mercia Ph&quot;,&quot;CompanyCode&quot;:47},{&quot;CompanyName&quot;:&quot;MetroHealth Medical Center&quot;,&quot;CompanyCode&quot;:218},{&quot;CompanyName&quot;:&quot;Millenium Pharmaceuticals&quot;,&quot;CompanyCode&quot;:88},{&quot;CompanyName&quot;:&quot;Mitos&quot;,&quot;CompanyCode&quot;:76},{&quot;CompanyName&quot;:&quot;Motus Pharmaceuticals, Inc.&quot;,&quot;CompanyCode&quot;:205},{&quot;CompanyName&quot;:&quot;Multi-Party Agreement&quot;,&quot;CompanyCode&quot;:0},{&quot;CompanyName&quot;:&quot;Mylan&quot;,&quot;CompanyCode&quot;:159},{&quot;CompanyName&quot;:&quot;N8 Medical LLC&quot;,&quot;CompanyCode&quot;:236},{&quot;CompanyName&quot;:&quot;N8 Medical, Inc.&quot;,&quot;CompanyCode&quot;:197},{&quot;CompanyName&quot;:&quot;NanoPass Technologies&quot;,&quot;CompanyCode&quot;:226},{&quot;CompanyName&quot;:&quot;NeoImmuneTech, Inc.&quot;,&quot;CompanyCode&quot;:239},{&quot;CompanyName&quot;:&quot;Neostem&quot;,&quot;CompanyCode&quot;:153},{&quot;CompanyName&quot;:&quot;Nestle Health Science&quot;,&quot;CompanyCode&quot;:227},{&quot;CompanyName&quot;:&quot;Neumedicines&quot;,&quot;CompanyCode&quot;:101},{&quot;CompanyName&quot;:&quot;Neurocrine Biosciences&quot;,&quot;CompanyCode&quot;:14},{&quot;CompanyName&quot;:&quot;NIDDK&quot;,&quot;CompanyCode&quot;:99},{&quot;CompanyName&quot;:&quot;Nippon Kayaku&quot;,&quot;CompanyCode&quot;:23},{&quot;CompanyName&quot;:&quot;Novartis&quot;,&quot;CompanyCode&quot;:20},{&quot;CompanyName&quot;:&quot;NovaScreen Biosciences Corporation&quot;,&quot;CompanyCode&quot;:58},{&quot;CompanyName&quot;:&quot;Novelos&quot;,&quot;CompanyCode&quot;:39},{&quot;CompanyName&quot;:&quot;NOW Foods&quot;,&quot;CompanyCode&quot;:74},{&quot;CompanyName&quot;:&quot;Nuvo Research AG&quot;,&quot;CompanyCode&quot;:161},{&quot;CompanyName&quot;:&quot;Ocera Therapeutics&quot;,&quot;CompanyCode&quot;:111},{&quot;CompanyName&quot;:&quot;OHSU Center for Regenerative Medicine in Oregon&quot;,&quot;CompanyCode&quot;:200},{&quot;CompanyName&quot;:&quot;Onconova Therapeutics, Inc.&quot;,&quot;CompanyCode&quot;:29},{&quot;CompanyName&quot;:&quot;Onyx&quot;,&quot;CompanyCode&quot;:133},{&quot;CompanyName&quot;:&quot;Ore Pharmaceuticals&quot;,&quot;CompanyCode&quot;:112},{&quot;CompanyName&quot;:&quot;Osiris Therapeutics, Inc.&quot;,&quot;CompanyCode&quot;:51},{&quot;CompanyName&quot;:&quot;Ossium Health, Inc.&quot;,&quot;CompanyCode&quot;:221},{&quot;CompanyName&quot;:&quot;Ovation Pharmaceuticals&quot;,&quot;CompanyCode&quot;:40},{&quot;CompanyName&quot;:&quot;Paloma Pharmaceuticals, Inc.&quot;,&quot;CompanyCode&quot;:174},{&quot;CompanyName&quot;:&quot;ParinGenix&quot;,&quot;CompanyCode&quot;:164},{&quot;CompanyName&quot;:&quot;Pennsylvania State University&quot;,&quot;CompanyCode&quot;:215},{&quot;CompanyName&quot;:&quot;Pfizer&quot;,&quot;CompanyCode&quot;:17},{&quot;CompanyName&quot;:&quot;Pharmaceutical Product Development (PPD)&quot;,&quot;CompanyCode&quot;:180},{&quot;CompanyName&quot;:&quot;Pharmacia Diagnostics AB&quot;,&quot;CompanyCode&quot;:78},{&quot;CompanyName&quot;:&quot;Phylogix, Inc.&quot;,&quot;CompanyCode&quot;:91},{&quot;CompanyName&quot;:&quot;Physiocrines&quot;,&quot;CompanyCode&quot;:149},{&quot;CompanyName&quot;:&quot;PhysioGenix, Inc.&quot;,&quot;CompanyCode&quot;:166},{&quot;CompanyName&quot;:&quot;Pinnacle Biologics, Inc.&quot;,&quot;CompanyCode&quot;:177},{&quot;CompanyName&quot;:&quot;Pluristem Therapeutic Inc.&quot;,&quot;CompanyCode&quot;:162},{&quot;CompanyName&quot;:&quot;Point Therapeutics&quot;,&quot;CompanyCode&quot;:35},{&quot;CompanyName&quot;:&quot;PowderJect Technologies Limited&quot;,&quot;CompanyCode&quot;:92},{&quot;CompanyName&quot;:&quot;ProMetic BioSciences, Inc.&quot;,&quot;CompanyCode&quot;:172},{&quot;CompanyName&quot;:&quot;Proteome Systems&quot;,&quot;CompanyCode&quot;:37},{&quot;CompanyName&quot;:&quot;Radiation Control Technologies, Inc&quot;,&quot;CompanyCode&quot;:168},{&quot;CompanyName&quot;:&quot;Radix Therapeutics&quot;,&quot;CompanyCode&quot;:160},{&quot;CompanyName&quot;:&quot;Reata Pharmaceuticals, Inc.&quot;,&quot;CompanyCode&quot;:93},{&quot;CompanyName&quot;:&quot;Regeneron&quot;,&quot;CompanyCode&quot;:208},{&quot;CompanyName&quot;:&quot;Repligen&quot;,&quot;CompanyCode&quot;:15},{&quot;CompanyName&quot;:&quot;RestorGenix, Inc.&quot;,&quot;CompanyCode&quot;:192},{&quot;CompanyName&quot;:&quot;Rhythm Pharmaceuticals, Inc&quot;,&quot;CompanyCode&quot;:198},{&quot;CompanyName&quot;:&quot;Roche&quot;,&quot;CompanyCode&quot;:11},{&quot;CompanyName&quot;:&quot;Rx Bio, Inc.&quot;,&quot;CompanyCode&quot;:34},{&quot;CompanyName&quot;:&quot;Sangstat Medical Corporation&quot;,&quot;CompanyCode&quot;:5},{&quot;CompanyName&quot;:&quot;Sanofi, Inc.&quot;,&quot;CompanyCode&quot;:173},{&quot;CompanyName&quot;:&quot;Seattle Genetics&quot;,&quot;CompanyCode&quot;:211},{&quot;CompanyName&quot;:&quot;SEPPIC Inc&quot;,&quot;CompanyCode&quot;:148},{&quot;CompanyName&quot;:&quot;Sloan Kettering Institute for Cancer Research&quot;,&quot;CompanyCode&quot;:110},{&quot;CompanyName&quot;:&quot;SmithKline Beecham Corporation&quot;,&quot;CompanyCode&quot;:125},{&quot;CompanyName&quot;:&quot;Soligenix, Inc&quot;,&quot;CompanyCode&quot;:143},{&quot;CompanyName&quot;:&quot;Sologenix, Inc&quot;,&quot;CompanyCode&quot;:142},{&quot;CompanyName&quot;:&quot;Southwest Research Institute&quot;,&quot;CompanyCode&quot;:228},{&quot;CompanyName&quot;:&quot;SRI  International&quot;,&quot;CompanyCode&quot;:94},{&quot;CompanyName&quot;:&quot;St. Camillus Medical&quot;,&quot;CompanyCode&quot;:95},{&quot;CompanyName&quot;:&quot;Synedgen&quot;,&quot;CompanyCode&quot;:190},{&quot;CompanyName&quot;:&quot;TajCo Inc&quot;,&quot;CompanyCode&quot;:103},{&quot;CompanyName&quot;:&quot;Targazyme&quot;,&quot;CompanyCode&quot;:233},{&quot;CompanyName&quot;:&quot;TDeltaS&quot;,&quot;CompanyCode&quot;:223},{&quot;CompanyName&quot;:&quot;Temple University&quot;,&quot;CompanyCode&quot;:217},{&quot;CompanyName&quot;:&quot;Terapio&quot;,&quot;CompanyCode&quot;:75},{&quot;CompanyName&quot;:&quot;Teva&quot;,&quot;CompanyCode&quot;:21},{&quot;CompanyName&quot;:&quot;The Institutes for Pharmaceutical Discovery&quot;,&quot;CompanyCode&quot;:89},{&quot;CompanyName&quot;:&quot;TikoMed AB&quot;,&quot;CompanyCode&quot;:50},{&quot;CompanyName&quot;:&quot;Tobira Therapeutics, Inc.&quot;,&quot;CompanyCode&quot;:206},{&quot;CompanyName&quot;:&quot;Tricorder Diagnostics&quot;,&quot;CompanyCode&quot;:141},{&quot;CompanyName&quot;:&quot;Tunitas Therapeutics, Inc.&quot;,&quot;CompanyCode&quot;:138},{&quot;CompanyName&quot;:&quot;UCB Pharma S.A. Belgium&quot;,&quot;CompanyCode&quot;:209},{&quot;CompanyName&quot;:&quot;UCLA&quot;,&quot;CompanyCode&quot;:158},{&quot;CompanyName&quot;:&quot;University of Arkansas for Medical Sciences&quot;,&quot;CompanyCode&quot;:54},{&quot;CompanyName&quot;:&quot;University of California San Diego&quot;,&quot;CompanyCode&quot;:213},{&quot;CompanyName&quot;:&quot;University of Emory&quot;,&quot;CompanyCode&quot;:179},{&quot;CompanyName&quot;:&quot;University of Florida&quot;,&quot;CompanyCode&quot;:187},{&quot;CompanyName&quot;:&quot;University of Kansas Medical Center&quot;,&quot;CompanyCode&quot;:240},{&quot;CompanyName&quot;:&quot;University of North Carolina at Chapel Hill&quot;,&quot;CompanyCode&quot;:128},{&quot;CompanyName&quot;:&quot;University of Pennsylvania&quot;,&quot;CompanyCode&quot;:175},{&quot;CompanyName&quot;:&quot;University of Pittsburgh Medical Center&quot;,&quot;CompanyCode&quot;:225},{&quot;CompanyName&quot;:&quot;University of Rochester&quot;,&quot;CompanyCode&quot;:30},{&quot;CompanyName&quot;:&quot;University of Utah&quot;,&quot;CompanyCode&quot;:129},{&quot;CompanyName&quot;:&quot;University of Virginia&quot;,&quot;CompanyCode&quot;:181},{&quot;CompanyName&quot;:&quot;US Biotest, Inc.&quot;,&quot;CompanyCode&quot;:120},{&quot;CompanyName&quot;:&quot;Vifor&quot;,&quot;CompanyCode&quot;:144},{&quot;CompanyName&quot;:&quot;ViraCor&quot;,&quot;CompanyCode&quot;:31},{&quot;CompanyName&quot;:&quot;Vitaeris&quot;,&quot;CompanyCode&quot;:246},{&quot;CompanyName&quot;:&quot;Wake Forest School of Medicine&quot;,&quot;CompanyCode&quot;:176},{&quot;CompanyName&quot;:&quot;Wyeth Pharmaceuticals&quot;,&quot;CompanyCode&quot;:113},{&quot;CompanyName&quot;:&quot;Wyeth-Ayerst Research&quot;,&quot;CompanyCode&quot;:6},{&quot;CompanyName&quot;:&quot;Xoma&quot;,&quot;CompanyCode&quot;:109},{&quot;CompanyName&quot;:&quot;Yale University&quot;,&quot;CompanyCode&quot;:216},{&quot;CompanyName&quot;:&quot;zz-test1&quot;,&quot;CompanyCode&quot;:250},{&quot;CompanyName&quot;:&quot;zz-test2&quot;,&quot;CompanyCode&quot;:251},{&quot;CompanyName&quot;:&quot;zz-test-firefox&quot;,&quot;CompanyCode&quot;:252}],&quot;sourceID&quot;:79967188,&quot;success&quot;:true,&quot;totalRows&quot;:227,&quot;totalSourceRows&quot;:227}

    var companyArray = companyArr.myRows;



    
    var productArr = {&quot;actualRows&quot;:254,&quot;filteredRows&quot;:254,&quot;myRows&quot;:[{&quot;ProductName&quot;:&quot;3-D tissues (platform technologies)&quot;},{&quot;ProductName&quot;:&quot;AB103&quot;},{&quot;ProductName&quot;:&quot;Actemra (tocilizumab)&quot;},{&quot;ProductName&quot;:&quot;Adipose-Derived Stem Cells &amp; Regenerative Cells&quot;},{&quot;ProductName&quot;:&quot;Advair Diskus&quot;},{&quot;ProductName&quot;:&quot;AEOL 10150&quot;},{&quot;ProductName&quot;:&quot;Aflunov (MF59-adjuvanted)&quot;},{&quot;ProductName&quot;:&quot;Aflunov (MF59-unadjuvanted)&quot;},{&quot;ProductName&quot;:&quot;Ajulemic Acid&quot;},{&quot;ProductName&quot;:&quot;Alutard SQ&quot;},{&quot;ProductName&quot;:&quot;ALXN-4100 (TPO)&quot;},{&quot;ProductName&quot;:&quot;Amb a 1&quot;},{&quot;ProductName&quot;:&quot;Amevive (alefacept)&quot;},{&quot;ProductName&quot;:&quot;AMG 157 (anti-TSLP)&quot;},{&quot;ProductName&quot;:&quot;AMG 714  (anti-IL-15 antibody)&quot;},{&quot;ProductName&quot;:&quot;Amifostine (WR1064)&quot;},{&quot;ProductName&quot;:&quot;Amino Acid Cocktail&quot;},{&quot;ProductName&quot;:&quot;angiotensin peptides&quot;},{&quot;ProductName&quot;:&quot;Anti IL-17 mab&quot;},{&quot;ProductName&quot;:&quot;Anti-ceramide antibody&quot;},{&quot;ProductName&quot;:&quot;Anti-Il12&quot;},{&quot;ProductName&quot;:&quot;Anti-IL-5 (mepolizumab)&quot;},{&quot;ProductName&quot;:&quot;anti-TGF-B antibody&quot;},{&quot;ProductName&quot;:&quot;anti-TNF&quot;},{&quot;ProductName&quot;:&quot;AR101&quot;},{&quot;ProductName&quot;:&quot;ARA 290&quot;},{&quot;ProductName&quot;:&quot;Aralast NP&quot;},{&quot;ProductName&quot;:&quot;ARQ 501&quot;},{&quot;ProductName&quot;:&quot;ARQ 900&quot;},{&quot;ProductName&quot;:&quot;ASMase pathway in GI tract&quot;},{&quot;ProductName&quot;:&quot;Assay Reagent Kits&quot;},{&quot;ProductName&quot;:&quot;AST-120&quot;},{&quot;ProductName&quot;:&quot;Atorvastatin&quot;},{&quot;ProductName&quot;:&quot;Avonex&quot;},{&quot;ProductName&quot;:&quot;BAC-3&quot;},{&quot;ProductName&quot;:&quot;Baminercept&quot;},{&quot;ProductName&quot;:&quot;BB3&quot;},{&quot;ProductName&quot;:&quot;BCN512&quot;},{&quot;ProductName&quot;:&quot;Belatacept (LEA29Y)&quot;},{&quot;ProductName&quot;:&quot;Belimumab&quot;},{&quot;ProductName&quot;:&quot;BIO300&quot;},{&quot;ProductName&quot;:&quot;BMS-188667&quot;},{&quot;ProductName&quot;:&quot;BMS-224818&quot;},{&quot;ProductName&quot;:&quot;BMS-224819&quot;},{&quot;ProductName&quot;:&quot;Bone Marrow bank&quot;},{&quot;ProductName&quot;:&quot;BPI&quot;},{&quot;ProductName&quot;:&quot;Brentuximab (Adcentris)&quot;},{&quot;ProductName&quot;:&quot;C22TT&quot;},{&quot;ProductName&quot;:&quot;C2E5&quot;},{&quot;ProductName&quot;:&quot;Ca-DTPA&quot;},{&quot;ProductName&quot;:&quot;Campath-1H&quot;},{&quot;ProductName&quot;:&quot;Captopril&quot;},{&quot;ProductName&quot;:&quot;Carfilzomib&quot;},{&quot;ProductName&quot;:&quot;Cat allergen extracts&quot;},{&quot;ProductName&quot;:&quot;Cat Dander&quot;},{&quot;ProductName&quot;:&quot;Cat-PAD&quot;},{&quot;ProductName&quot;:&quot;Cat-PAD &amp; placebo for Cat-PAD&quot;},{&quot;ProductName&quot;:&quot;CBLB-502&quot;},{&quot;ProductName&quot;:&quot;CBLB-612&quot;},{&quot;ProductName&quot;:&quot;CD 34 Reagent Kits&quot;},{&quot;ProductName&quot;:&quot;CD3+\/CD19+ depleted bone marrow&quot;},{&quot;ProductName&quot;:&quot;CDK4\/6 Inhibitors&quot;},{&quot;ProductName&quot;:&quot;CDX-301&quot;},{&quot;ProductName&quot;:&quot;Cenicriviroc&quot;},{&quot;ProductName&quot;:&quot;Ceragenins (Cationic Steroid Antibiotics)&quot;},{&quot;ProductName&quot;:&quot;Ceramide&quot;},{&quot;ProductName&quot;:&quot;CG53135-05 (FGF-20)&quot;},{&quot;ProductName&quot;:&quot;Chalcone derivative (2-trifluoromethyl-2\u0027-methoxychalone)&quot;},{&quot;ProductName&quot;:&quot;Characterized Peanut Allergen&quot;},{&quot;ProductName&quot;:&quot;Chemet&quot;},{&quot;ProductName&quot;:&quot;CLT008&quot;},{&quot;ProductName&quot;:&quot;Cockroach Allergen Extract&quot;},{&quot;ProductName&quot;:&quot;congener compounds&quot;},{&quot;ProductName&quot;:&quot;CTLA4-IgG4m&quot;},{&quot;ProductName&quot;:&quot;DeltaG&quot;},{&quot;ProductName&quot;:&quot;Deoxyspergualin&quot;},{&quot;ProductName&quot;:&quot;DIM (3&quot;},{&quot;ProductName&quot;:&quot;DM99&quot;},{&quot;ProductName&quot;:&quot;Dupilumab&quot;},{&quot;ProductName&quot;:&quot;EA-230&quot;},{&quot;ProductName&quot;:&quot;ECIS Based Automated Stem cell functionality Assay&quot;},{&quot;ProductName&quot;:&quot;EDL2000&quot;},{&quot;ProductName&quot;:&quot;EFK2 inhibitor&quot;},{&quot;ProductName&quot;:&quot;EMP-123&quot;},{&quot;ProductName&quot;:&quot;Enalapril&quot;},{&quot;ProductName&quot;:&quot;Enterade-R&quot;},{&quot;ProductName&quot;:&quot;EntericSorb&quot;},{&quot;ProductName&quot;:&quot;Epi-1A&quot;},{&quot;ProductName&quot;:&quot;EPI-743&quot;},{&quot;ProductName&quot;:&quot;EpiPen&quot;},{&quot;ProductName&quot;:&quot;EpiPen Jr.&quot;},{&quot;ProductName&quot;:&quot;Epix-107&quot;},{&quot;ProductName&quot;:&quot;Epix-111&quot;},{&quot;ProductName&quot;:&quot;Epix-112&quot;},{&quot;ProductName&quot;:&quot;Erythropoietin&quot;},{&quot;ProductName&quot;:&quot;Etanercept&quot;},{&quot;ProductName&quot;:&quot;EUK-189&quot;},{&quot;ProductName&quot;:&quot;Fab59&quot;},{&quot;ProductName&quot;:&quot;FG-4539&quot;},{&quot;ProductName&quot;:&quot;Fibroblast Growth Factor peptide (FGF-P)&quot;},{&quot;ProductName&quot;:&quot;Flaxseed (Z6748)&quot;},{&quot;ProductName&quot;:&quot;Flonase Nasal Spray&quot;},{&quot;ProductName&quot;:&quot;Flovent Diskus&quot;},{&quot;ProductName&quot;:&quot;Flt3 Receptor-Interacting Lectin (FRIL)&quot;},{&quot;ProductName&quot;:&quot;Genistein&quot;},{&quot;ProductName&quot;:&quot;Glucopyranosyl Lipid A (GLA)&quot;},{&quot;ProductName&quot;:&quot;Glycerinated Peanut Allergenic Extract&quot;},{&quot;ProductName&quot;:&quot;Granexin Gel&quot;},{&quot;ProductName&quot;:&quot;Grazax&quot;},{&quot;ProductName&quot;:&quot;GTS-21&quot;},{&quot;ProductName&quot;:&quot;H1N1 (Influenza A) 2009 S-01v Vaccine&quot;},{&quot;ProductName&quot;:&quot;H5N1 Flu Adjuvant&quot;},{&quot;ProductName&quot;:&quot;H5N1 Flu Vaccine&quot;},{&quot;ProductName&quot;:&quot;Hemp Oil&quot;},{&quot;ProductName&quot;:&quot;hOKT3 (anti-CD3)&quot;},{&quot;ProductName&quot;:&quot;HOPO derivatives&quot;},{&quot;ProductName&quot;:&quot;House dust mite- mix&quot;},{&quot;ProductName&quot;:&quot;Human iPS cell manufacturing platform&quot;},{&quot;ProductName&quot;:&quot;human recombinant thrombopoietin&quot;},{&quot;ProductName&quot;:&quot;hyaluronic acid&quot;},{&quot;ProductName&quot;:&quot;IDEC-131 (anti-CD154)&quot;},{&quot;ProductName&quot;:&quot;IL-12&quot;},{&quot;ProductName&quot;:&quot;Immune Globulin Intravenous&quot;},{&quot;ProductName&quot;:&quot;Immunostimnlary oligodeoxyribonucleotide conjugate (AIC)&quot;},{&quot;ProductName&quot;:&quot;Imprime WGP&quot;},{&quot;ProductName&quot;:&quot;Infliximab&quot;},{&quot;ProductName&quot;:&quot;Intradermal Fcy1-Fel d1 fusion protein (GFD)&quot;},{&quot;ProductName&quot;:&quot;IPW5371&quot;},{&quot;ProductName&quot;:&quot;Islet&quot;},{&quot;ProductName&quot;:&quot;Isolex 300i Disposable Sets&quot;},{&quot;ProductName&quot;:&quot;Isolex 300i Magneticcell Selection System Machine&quot;},{&quot;ProductName&quot;:&quot;Isothiouronium derivatives&quot;},{&quot;ProductName&quot;:&quot;JBT-101&quot;},{&quot;ProductName&quot;:&quot;Keyhole Limpet Hemocyanin (KLH)&quot;},{&quot;ProductName&quot;:&quot;Leucotropin (GM-CSF)&quot;},{&quot;ProductName&quot;:&quot;Leukine&quot;},{&quot;ProductName&quot;:&quot;LHRH-Ant&quot;},{&quot;ProductName&quot;:&quot;Lovastatin Tablets&quot;},{&quot;ProductName&quot;:&quot;Low Molecular Weight Dextran Sulfate&quot;},{&quot;ProductName&quot;:&quot;Lulizumab&quot;},{&quot;ProductName&quot;:&quot;MAS-1&quot;},{&quot;ProductName&quot;:&quot;Maxy-G34&quot;},{&quot;ProductName&quot;:&quot;MC8  inserts&quot;},{&quot;ProductName&quot;:&quot;MEDI-507&quot;},{&quot;ProductName&quot;:&quot;Mesenchymal Stem Cells&quot;},{&quot;ProductName&quot;:&quot;MGA-031&quot;},{&quot;ProductName&quot;:&quot;MicronJet600 needles&quot;},{&quot;ProductName&quot;:&quot;MIS416&quot;},{&quot;ProductName&quot;:&quot;molecules&quot;},{&quot;ProductName&quot;:&quot;Montanide ISA 51 VG&quot;},{&quot;ProductName&quot;:&quot;Montanide ISA 51 VG Sterile&quot;},{&quot;ProductName&quot;:&quot;mouse Myeloid Progenitor Cells (mMPc)&quot;},{&quot;ProductName&quot;:&quot;Mozobil&quot;},{&quot;ProductName&quot;:&quot;MultiPep&quot;},{&quot;ProductName&quot;:&quot;MultiStem&quot;},{&quot;ProductName&quot;:&quot;N\/A&quot;},{&quot;ProductName&quot;:&quot;Nadolol&quot;},{&quot;ProductName&quot;:&quot;NBI-5788&quot;},{&quot;ProductName&quot;:&quot;Neulasta (pegfilgrastim)&quot;},{&quot;ProductName&quot;:&quot;Neumune (androst-5-ene-3b&quot;},{&quot;ProductName&quot;:&quot;Neupogen&quot;},{&quot;ProductName&quot;:&quot;Nilotinib&quot;},{&quot;ProductName&quot;:&quot;Non-Fat Dry Milk (NFD Milk)&quot;},{&quot;ProductName&quot;:&quot;NOV-002&quot;},{&quot;ProductName&quot;:&quot;Nplate&quot;},{&quot;ProductName&quot;:&quot;NRF2 upregulators&quot;},{&quot;ProductName&quot;:&quot;NU206 (R-Spondin 1)&quot;},{&quot;ProductName&quot;:&quot;Nucala (mepolizumab)&quot;},{&quot;ProductName&quot;:&quot;ODSH&quot;},{&quot;ProductName&quot;:&quot;Oligonucleotide morpholino to CD47&quot;},{&quot;ProductName&quot;:&quot;ON 01210.Na (Ex-Rad)&quot;},{&quot;ProductName&quot;:&quot;Oral DTPA&quot;},{&quot;ProductName&quot;:&quot;Orencia (abatacept)&quot;},{&quot;ProductName&quot;:&quot;Otelixizumab&quot;},{&quot;ProductName&quot;:&quot;otelixizumab and sirukumab&quot;},{&quot;ProductName&quot;:&quot;P529&quot;},{&quot;ProductName&quot;:&quot;PAAG&quot;},{&quot;ProductName&quot;:&quot;PBI-1402&quot;},{&quot;ProductName&quot;:&quot;Peanut Protein Oral Immunotherapy (AR101)&quot;},{&quot;ProductName&quot;:&quot;PEGylation IgG fusion&quot;},{&quot;ProductName&quot;:&quot;phosphonol&quot;},{&quot;ProductName&quot;:&quot;Physiocrines&quot;},{&quot;ProductName&quot;:&quot;placebo for Abatacept&quot;},{&quot;ProductName&quot;:&quot;placebo for AMG 157&quot;},{&quot;ProductName&quot;:&quot;placebo for Aralast NP&quot;},{&quot;ProductName&quot;:&quot;placebo for Atorvastatin&quot;},{&quot;ProductName&quot;:&quot;placebo for Cat-PAD&quot;},{&quot;ProductName&quot;:&quot;placebo for Cockroach Allergen Extract&quot;},{&quot;ProductName&quot;:&quot;placebo for Flovent Diskus&quot;},{&quot;ProductName&quot;:&quot;placebo for Nadolol&quot;},{&quot;ProductName&quot;:&quot;placebo for Rituxan&quot;},{&quot;ProductName&quot;:&quot;placebo for Viaskin Peanut&quot;},{&quot;ProductName&quot;:&quot;placebo for Xolair&quot;},{&quot;ProductName&quot;:&quot;PLacental eXpanded (PLX-RAD) cells&quot;},{&quot;ProductName&quot;:&quot;Pleiotropin (PTN)&quot;},{&quot;ProductName&quot;:&quot;Plerixafor (AMD3100)&quot;},{&quot;ProductName&quot;:&quot;PoC Biodosimeter&quot;},{&quot;ProductName&quot;:&quot;PPARgamma ligands&quot;},{&quot;ProductName&quot;:&quot;Pre-Implantation Factor (PIF)&quot;},{&quot;ProductName&quot;:&quot;PREMEA&quot;},{&quot;ProductName&quot;:&quot;Prograf (tacrolimus)&quot;},{&quot;ProductName&quot;:&quot;Proleukin (IL-2)&quot;},{&quot;ProductName&quot;:&quot;Radilex&quot;},{&quot;ProductName&quot;:&quot;Ragweed Allergen Immunostimulatory Sequence Conjugate&quot;},{&quot;ProductName&quot;:&quot;Rapamune (sirolimus)&quot;},{&quot;ProductName&quot;:&quot;rBPI-21&quot;},{&quot;ProductName&quot;:&quot;Regulatory T cells (Treg)&quot;},{&quot;ProductName&quot;:&quot;Relamorelin (RM-131)&quot;},{&quot;ProductName&quot;:&quot;RG2077 (CTLA4-IG)&quot;},{&quot;ProductName&quot;:&quot;Rituxan (rituximab)&quot;},{&quot;ProductName&quot;:&quot;RLIP76&quot;},{&quot;ProductName&quot;:&quot;RM-131&quot;},{&quot;ProductName&quot;:&quot;RN168&quot;},{&quot;ProductName&quot;:&quot;RWJ-800088&quot;},{&quot;ProductName&quot;:&quot;Rx100&quot;},{&quot;ProductName&quot;:&quot;Rx101&quot;},{&quot;ProductName&quot;:&quot;Secoisolariciresinaol diglucosides (SDGs)&quot;},{&quot;ProductName&quot;:&quot;Secukinumab (anti-IL-17)&quot;},{&quot;ProductName&quot;:&quot;SGX202 (oral beclomethasone dipropionate or BDP)&quot;},{&quot;ProductName&quot;:&quot;SGX942&quot;},{&quot;ProductName&quot;:&quot;Sphingosine&quot;},{&quot;ProductName&quot;:&quot;Standardized Cat Hair Extract&quot;},{&quot;ProductName&quot;:&quot;statins&quot;},{&quot;ProductName&quot;:&quot;Synexan (SP001)&quot;},{&quot;ProductName&quot;:&quot;synthetic triterpenoids RTA 401&quot;},{&quot;ProductName&quot;:&quot;synthetic triterpenoids RTA 402&quot;},{&quot;ProductName&quot;:&quot;Tabalostat&quot;},{&quot;ProductName&quot;:&quot;TBO-filgrastim&quot;},{&quot;ProductName&quot;:&quot;T-cell epitode peptides&quot;},{&quot;ProductName&quot;:&quot;Tempol&quot;},{&quot;ProductName&quot;:&quot;Teplizumab&quot;},{&quot;ProductName&quot;:&quot;Thymoglobulin&quot;},{&quot;ProductName&quot;:&quot;Timothy grass&quot;},{&quot;ProductName&quot;:&quot;TP508&quot;},{&quot;ProductName&quot;:&quot;TPOm&quot;},{&quot;ProductName&quot;:&quot;Trichuris suis ova (TSO)&quot;},{&quot;ProductName&quot;:&quot;TXA127 (formulated angiotensin 1-7)&quot;},{&quot;ProductName&quot;:&quot;Unadjuvanted H1N1 Influenza Virus Vaccine&quot;},{&quot;ProductName&quot;:&quot;Velafermin&quot;},{&quot;ProductName&quot;:&quot;Velcade (bortezomib)&quot;},{&quot;ProductName&quot;:&quot;Ventolin HFA (albuterol sulfate)&quot;},{&quot;ProductName&quot;:&quot;Very Small Embryonic-Like cells (VSELs)&quot;},{&quot;ProductName&quot;:&quot;Viaskin Peanut&quot;},{&quot;ProductName&quot;:&quot;Viral Panel (human samples)&quot;},{&quot;ProductName&quot;:&quot;Vitamin D3&quot;},{&quot;ProductName&quot;:&quot;WF10&quot;},{&quot;ProductName&quot;:&quot;Xolair (omalizumab)&quot;},{&quot;ProductName&quot;:&quot;XOMA 052&quot;},{&quot;ProductName&quot;:&quot;Yel 002&quot;},{&quot;ProductName&quot;:&quot;Yel\/Rad 1\/2&quot;},{&quot;ProductName&quot;:&quot;Yel\/Rad 1\/2 derivatives&quot;},{&quot;ProductName&quot;:&quot;Zenapax (daclizumab)&quot;},{&quot;ProductName&quot;:&quot;Zn-DTPA&quot;},{&quot;ProductName&quot;:&quot;Zortress (Everolimus)&quot;}],&quot;sourceID&quot;:80013981,&quot;success&quot;:true,&quot;totalRows&quot;:254,&quot;totalSourceRows&quot;:254}

    var productArray = productArr.myRows;












function setAgreementTypeList () {
    for (var item, i = 0; item = agreementTypeArray[i++];) {
        var agreementType = item.AgreementType;
        $(&quot;#_1_1_26_1&quot;).append($(&quot;&lt;option>&lt;/option>&quot;).val(agreementType).html(agreementType));
    }
}


function setCompanyList (controlID) {
    for (var item, i = 0; item = companyArray[i++];) {
        var company = item.CompanyName;
        $(&quot;#&quot; + controlID).append($(&quot;&lt;option>&lt;/option>&quot;).val(company).html(company));
    }
}


function setProductList (controlID) {
    for (var item, i = 0; item = productArray[i++];) {
        var product = item.ProductName;
        $(&quot;#&quot; + controlID).append($(&quot;&lt;option>&lt;/option>&quot;).val(product).html(product));
    }
}



function setCompanyInfo(controlID) {

    
    var id = parseInt(controlID.replace(&quot;ddCompany&quot;,&quot;&quot;));
    var selectedCompany = $(&quot;#&quot; + controlID).val();
    
    
    $(&quot;#_1_1_21_&quot; + id + &quot;_23_1&quot;).val(selectedCompany);
    
    
    var coCurrVal   = $(&quot;#_1_1_29_1&quot;).val();
    var coNewVal    = &quot;&quot;;
    if (coCurrVal == &quot;&quot;) {
        coNewVal = selectedCompany;
    } else {
        coNewVal = coCurrVal + &quot;|&quot; + selectedCompany;
    }
    $(&quot;#_1_1_29_1&quot;).val(coNewVal);
    
    
    generateAgreementNumber(controlID, id);
    
}


function setProductInfo(controlID) {

    var selectedProduct = $(&quot;#&quot; + controlID).val();
    
    
    var idx     = controlID.replace(&quot;ddProduct&quot;,&quot;&quot;);  
    var idArr   = idx.split(&quot;_&quot;);
    
    
    $(&quot;#_1_1_21_&quot; + idArr[0] + &quot;_24_&quot; + idArr[1]).val(selectedProduct);
    
}












function toggleMultiCompany(x) {
    
    if (x == &quot;Yes&quot;) {
        $(&quot;#addrow_1_1_21_1&quot;).show();
    } else {
        $(&quot;#addrow_1_1_21_1&quot;).hide();
        
        clearCompanyInfoAll();     
    }
}


function clearCompanyInfoAll() {
    
    clearCompanyInfoRow(&quot;ddCompany1&quot;, 1, 23)
    clearCompanyInfoRow(&quot;ddProduct1&quot;, 1, 24)
    
    
    for (i = 2; i &lt;= 3; i++) { 
        $(&quot;#tr_1_1_21_&quot; + i).remove();
    }      
    
    
    $(&quot;#_1_1_29_1&quot;).val(&quot;&quot;);
    $(&quot;#_1_1_30_1&quot;).val(&quot;&quot;);
}


function clearCompanyInfoRow(controlID, row, controlIDSet) {
    
    $(&quot;#&quot; + controlID).val(&quot;&quot;);                                     
    $(&quot;#_1_1_21_&quot; + row + &quot;_&quot; + controlIDSet + &quot;_1&quot;).val(&quot;&quot;);       

    $(&quot;#spAgreementNumber&quot; + row).text(&quot;&quot;);                         
    $(&quot;#_1_1_21_&quot; + row + &quot;_22_1&quot;).val(&quot;&quot;);                         
    
}


function clearUserFields(id) {
	$('#' + id + '_ID').val('');
	$('#' + id + '_SavedName').val('');
}	















function generateAgreementNumber(controlID, id) {
    
    
    var selectedAgreementType = $(&quot;#_1_1_26_1&quot;).val();
    var agreementTypePrefix = &quot;&quot;;
    if (selectedAgreementType != &quot;&quot;) {
        for (var p, i = 0; p = agreementTypeArray[i++];) {
            if (p.AgreementType == selectedAgreementType) {
                agreementTypePrefix = p.AgreementTypePrefix;
            }
        }      
    } else {
        alert(&quot;Please select an Agreement Type.&quot;);
        clearCompanyInfoRow(controlID, id, 23);
        clearCompanyInfoRow(controlID, id, 24);
        return;
    }
    console.log(&quot;AgreementTypePrefix: &quot; + agreementTypePrefix);
    
    
    var currYear = new Date().getFullYear().toString().substr(-2);
    console.log(&quot;CurrentYear: &quot; + currYear);
    
    
    var selectedCompany = $(&quot;#&quot; + controlID).val();
    
    var companyCode = &quot;&quot;;
    for (var c, i = 0; c = companyArray[i++];) {
        if (c.CompanyName == selectedCompany) {
            companyCode = c.CompanyCode;
        }
    }    
    console.log(&quot;CompanyCode: &quot; + companyCode);
    
    
    var nxtSeqNum = &quot;&quot;;
    var agreeNumSearchString = agreementTypePrefix + &quot;.&quot; + currYear + &quot;.&quot; + companyCode;
    $.get(&quot;/livelink/llisapi.dll?func=ll&amp;objId=79977611&amp;objAction=RunReport&quot;, {
        AgreementNumberSearchString : agreeNumSearchString
    }, function(data){
        
        var topAgreeNum = data.trim();
        console.log(&quot;Ajax Response: &quot; + topAgreeNum);
        
        if (topAgreeNum != &quot;&quot;) {
            var topAgreeNumArray = topAgreeNum.split(&quot;.&quot;);
            
            if (topAgreeNumArray[3].length == 4) {
                
                var nxtSeqNumInt = parseInt(topAgreeNumArray[3]) + 1;
                nxtSeqNum = pad(nxtSeqNumInt.toString(), 4);
                console.log(&quot;nxtSeqNum: &quot; + nxtSeqNum);
                
            } else {
                
                
                
                
                
            }
        } else {
            
            nxtSeqNum = &quot;0001&quot;;
        }

        
        var agreementNumber = agreementTypePrefix + &quot;.&quot; + currYear + &quot;.&quot; + companyCode + &quot;.&quot; + nxtSeqNum;
        
        
        $(&quot;#_1_1_21_&quot; + id + &quot;_22_1&quot;).val(agreementNumber);
        $(&quot;#spAgreementNumber&quot; + id).text(agreementNumber);  
        
        
        
        var agCurrVal   = $(&quot;#_1_1_30_1&quot;).val();
        var agNewVal    = &quot;&quot;;
        if (agCurrVal == &quot;&quot;) {
            agNewVal = agreementNumber;
        } else {
            agNewVal = agCurrVal + &quot;|&quot; + agreementNumber;
        }
        $(&quot;#_1_1_30_1&quot;).val(agNewVal);           
        
        
    }); // End of Ajax call

}


function pad (str, max) {
  str = str.toString();
  return str.length &lt; max ? pad(&quot;0&quot; + str, max) : str;
}











function addCompanyRow(controlID) {
    
    var maxNumCompanies = 3;
    
	var lastRowObj  = $('#tblCompanies tr:last');
	var lastRowID   = lastRowObj.attr(&quot;id&quot;);
	
	
	lastRowID       = lastRowID.replace(/_24_[1-9][0-9]?/g, ''); 
	lastRowObj      = $('#' + lastRowID);
	
	var idArray     = lastRowID.split(&quot;_&quot;);
	var lastRow     = parseInt(idArray[4]);
    var nxtRow      = lastRow + 1;
    
    
    var currRowArray = controlID.split(&quot;_&quot;);
    var currRow = parseInt(currRowArray[4]);
    if (currRow != lastRow) {
        return;
    }
    
	var html =  '&lt;tr id=&quot;tr_1_1_21_{id}&quot;>'
	            + '&lt;td>'
                + '     &lt;select id=&quot;ddCompany{id}&quot; name=&quot;ddCompany{id}&quot; onchange=setCompanyInfo(this.id); style=&quot;width:330px;&quot;>&lt;option value=&quot;&quot;>- None -&lt;/option>&lt;/select>'
                + '     &lt;input type=&quot;hidden&quot; name=&quot;_1_1_21_{id}_23_1&quot; id=&quot;_1_1_21_{id}_23_1&quot; value=&quot;&quot; />'
                + '&lt;/td>'
                
				+ '&lt;td>'
                + '	    &lt;table id=&quot;tblProducts{id}&quot; cellpadding=&quot;0&quot; cellspacing=&quot;0&quot; width=&quot;100%&quot;>'
                + '	        &lt;tr id=&quot;tr_1_1_21_{id}_24_1&quot;>'
                + '	            &lt;td style=&quot;border:none; width:90%;&quot;>'
                + '    	            &lt;select id=&quot;ddProduct{id}_1&quot; name=&quot;ddProduct{id}_1&quot; onchange=&quot;setProductInfo(this.id);&quot; style=&quot;width:340px;&quot;>&lt;option value=&quot;&quot;>- None -&lt;/option>&lt;/select>'
                + '   	            &lt;input type=&quot;hidden&quot; name=&quot;_1_1_21_{id}_24_1&quot; id=&quot;_1_1_21_{id}_24_1&quot; value=&quot;&quot; />'    
                + '             &lt;/td>'
                + '             &lt;td style=&quot;border:none; width:10%;&quot; nowrap>'
                + '                 &lt;img id=&quot;addrow_1_1_21_{id}_24_1&quot; src=&quot;/img/add-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Add Row&quot; title=&quot;Add Row&quot; border=&quot;0&quot; onclick=&quot;addProductRow(this.id, {id})&quot;>'
                + '             &lt;/td>'
                + '         &lt;/tr>'
                + '     &lt;/table>'                
				+ '&lt;/td>'
				
				+ '&lt;td>'
				+ '     &lt;span id=&quot;spAgreementNumber{id}&quot;>&lt;/span>'
                + '     &lt;input type=&quot;hidden&quot; name=&quot;_1_1_21_{id}_22_1&quot; id=&quot;_1_1_21_{id}_22_1&quot; value=&quot;&quot; />'	
				+ '&lt;/td>'
				+ '&lt;td>'
    
	if (maxNumCompanies == nxtRow) {
	    html = html 
				+ '     &lt;img id=&quot;addrow_1_1_21_{id}&quot; src=&quot;/img/add-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Add Row&quot; title=&quot;Add Row&quot; border=&quot;0&quot; onclick=&quot;addCompanyRow(this.id)&quot; style=&quot;display:none;&quot;>'	    
				+ '     &lt;img id=&quot;delrow_1_1_21_{id}&quot; src=&quot;/img/delete-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Delete Row&quot; title=&quot;Delete Row&quot; border=&quot;0&quot; onclick=&quot;deleteCompanyRow(this.id)&quot;>'	            
	} else {
	    html = html
				+ '     &lt;img id=&quot;addrow_1_1_21_{id}&quot; src=&quot;/img/add-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Add Row&quot; title=&quot;Add Row&quot; border=&quot;0&quot; onclick=&quot;addCompanyRow(this.id)&quot;>'
				+ '     &lt;img id=&quot;delrow_1_1_21_{id}&quot; src=&quot;/img/delete-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Delete Row&quot; title=&quot;Delete Row&quot; border=&quot;0&quot; onclick=&quot;deleteCompanyRow(this.id)&quot;>'	    
	}
	
	html = html + '&lt;/td>'
				+ '&lt;/tr>';		
				
    html = html.replace(/{id}/g, nxtRow);
	lastRowObj.after( html );
	
	
	setCompanyList('ddCompany' + nxtRow);
	
	
	setProductList('ddProduct' + nxtRow + '_1');	
}


function addProductRow(controlID, idC) {
    
    var maxNumProducts = 5;
    
	var lastRowObj  = $('#tblProducts' + idC + ' tr:last');
	var lastRowID   = lastRowObj.attr(&quot;id&quot;);
	var idArray     = lastRowID.split(&quot;_&quot;);
	var lastRow     = parseInt(idArray[6]);
    var nxtRow      = lastRow + 1;
    
    
    var currRowArray = controlID.split(&quot;_&quot;);
    var currRow = parseInt(currRowArray[6]);
    if (currRow != lastRow) {
        return;
    }
    
	var html =  '&lt;tr id=&quot;tr_1_1_21_{idC}_24_{idP}&quot;>'
                + '     &lt;td style=&quot;border:none; width:90%;&quot;>'
                + '    	    &lt;select id=&quot;ddProduct{idC}_{idP}&quot; name=&quot;ddProduct{idC}_{idP}&quot; onchange=&quot;setProductInfo(this.id);&quot; style=&quot;width:340px;&quot;>&lt;option value=&quot;&quot;>- None -&lt;/option>&lt;/select>'
                + '         &lt;input type=&quot;hidden&quot; name=&quot;_1_1_21_{idC}_24_{idP}&quot; id=&quot;_1_1_21_{idC}_24_{idP}&quot; value=&quot;&quot; />'    
                + '     &lt;/td>'
				+ '     &lt;td style=&quot;border:none; width:10%&quot; nowrap>'
				
    
	if (maxNumProducts == nxtRow) {
	    html = html 
				+ '         &lt;img id=&quot;addrow_1_1_21_{idC}_24_{idP}&quot; src=&quot;/img/add-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Add Row&quot; title=&quot;Add Row&quot; border=&quot;0&quot; onclick=&quot;addProductRow(this.id, {idC})&quot; style=&quot;display:none;&quot;>'	    
				+ '         &lt;img id=&quot;delrow_1_1_21_{idC}_24_{idP}&quot; src=&quot;/img/delete-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Delete Row&quot; title=&quot;Delete Row&quot; border=&quot;0&quot; onclick=&quot;deleteProductRow(this.id)&quot;>'	            
	} else {
	    html = html
				+ '         &lt;img id=&quot;addrow_1_1_21_{idC}_24_{idP}&quot; src=&quot;/img/add-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Add Row&quot; title=&quot;Add Row&quot; border=&quot;0&quot; onclick=&quot;addProductRow(this.id, {idC})&quot;>'
				+ '         &lt;img id=&quot;delrow_1_1_21_{idC}_24_{idP}&quot; src=&quot;/img/delete-row.gif&quot; width=&quot;16&quot; height=&quot;16&quot; alt=&quot;Delete Row&quot; title=&quot;Delete Row&quot; border=&quot;0&quot; onclick=&quot;deleteProductRow(this.id)&quot;>'	    
	}
	
	html = html + '     &lt;/td>'
				+ '&lt;/tr>';		
				
    html = html.replace(/{idC}/g, idC);
    html = html.replace(/{idP}/g, nxtRow);
	lastRowObj.after( html );
	
	
	setProductList('ddProduct' + idC + '_' + nxtRow);
	
}


function deleteCompanyRow(id) {
    
    //console.log(&quot;Orig Del ID         = &quot; + id);
    
    var idArray = id.split(&quot;_&quot;);
    var trIdx   = parseInt(idArray[4]);
    
	
	var nxtTRRows = $(&quot;#tr_1_1_21_&quot; + (trIdx + 1));

    //console.log(&quot;nxtTRRow ID            = &quot; + $(nxtTRRows).attr('id') );

	$(&quot;#tr_1_1_21_&quot; + trIdx).remove();

	
	$.each(nxtTRRows, function(key, eachRow){

		var newRowIndx = trIdx + key;

		$(eachRow).find('input, img, div, select, span, td, tr, table').each(function(){

			var thisTRID = $(this).attr('id');
			var newVal = &quot;&quot;;

			if( thisTRID != undefined) {
			    
			    //console.log(&quot;newRowIndx         = &quot; + newRowIndx);
			    //console.log(&quot;Current Element ID = &quot; + thisTRID);

				
				if ( thisTRID.startsWith('ddCompany') ) {
				    newVal = thisTRID.replace(/ddCompany[1-9][0-9]?/g,'ddCompany' + newRowIndx);
				} else if ( thisTRID.startsWith('ddProduct') ) {
				    newVal = thisTRID.replace(/ddProduct[1-9][0-9]?/g,'ddProduct' + newRowIndx);  
				} else if ( thisTRID.startsWith('tblProducts') ) {
				    newVal = thisTRID.replace(/tblProducts[1-9][0-9]?/g,'tblProducts' + newRowIndx);  				    
				} else if ( thisTRID.startsWith('spAgreementNumber') ) {
				    newVal = thisTRID.replace(/spAgreementNumber[1-9][0-9]?/g,'spAgreementNumber' + newRowIndx);
				} else {
				    newVal = thisTRID.replace(/_1_21_[1-9][0-9]?/g,'_1_21_' + newRowIndx);
				}

				$(this).attr('id', newVal);
				$(this).attr('name', newVal);
				
				//console.log(&quot;newVal = &quot; + newVal);

			}
		});

		$(eachRow).attr(&quot;id&quot;,&quot;tr_1_1_21_&quot; + newRowIndx);

	});
	
	
	var lastRowObj  = $('#tblCompanies tr:last');
	var lastRowID   = lastRowObj.attr(&quot;id&quot;);
	
	
	lastRowID       = lastRowID.replace(/_24_[1-9][0-9]?/g, ''); 
	lastRowObj      = $('#' + lastRowID);	
	
	//console.log(lastRowID);
	
	var idArray     = lastRowID.split(&quot;_&quot;);
	var lastRow     = parseInt(idArray[4]);
	
	var lastRowPlusSign = &quot;addrow_1_1_21_&quot; + lastRow
	if ( $(&quot;#&quot; + lastRowPlusSign).css(&quot;display&quot;) == &quot;none&quot; ) {
		$(&quot;#&quot; + lastRowPlusSign).css(&quot;display&quot;,&quot;&quot;);
	}	
	
}


function deleteProductRow(id) {
    
    var idArray = id.split(&quot;_&quot;);
    var trIdC   = parseInt(idArray[4]);
    var trIdP   = parseInt(idArray[6]);
    
	
	var nxtTRRows = $(&quot;#tr_1_1_21_&quot; + trIdC + &quot;_24_&quot; + (trIdP + 1) );

	$(&quot;#tr_1_1_21_&quot; + trIdC + &quot;_24_&quot; + trIdP ).remove();

	
	$.each(nxtTRRows, function(key, eachRow){

		var newRowIndx = trIdP + key;

		$(eachRow).find('input, img, div, select, span, td').each(function(){

			var thisTRID = $(this).attr('id');
			var newVal = &quot;&quot;;

			if( thisTRID != undefined) {
			    
			    console.log(&quot;newRowIndx         = &quot; + newRowIndx);
			    console.log(&quot;Current Element ID = &quot; + thisTRID);

				
				if ( thisTRID.startsWith('ddProduct') ) {
				    newVal = thisTRID.replace(/_[1-9][0-9]?/g,'_' + newRowIndx);
				} else {
				    newVal = thisTRID.replace(/_1_24_[1-9][0-9]?/g,'_1_24_' + newRowIndx);
				}

				$(this).attr('id', newVal);
				$(this).attr('name', newVal);
				
				//console.log(&quot;newVal = &quot; + newVal);

			}
		});

		$(eachRow).attr(&quot;id&quot;,&quot;tr_1_1_21_&quot; + trIdC + &quot;_24_&quot; + newRowIndx);

	});
	
	
	var lastRowObj  = $('#tblProducts' + trIdC + ' tr:last');
	var lastRowID   = lastRowObj.attr(&quot;id&quot;);
	var idArray     = lastRowID.split(&quot;_&quot;);
	var lastRow     = parseInt(idArray[6]);
	
	var lastRowPlusSign = &quot;addrow_1_1_21_&quot; + trIdC + &quot;_24_&quot; + trIdP;
	if ( $(&quot;#&quot; + lastRowPlusSign).css(&quot;display&quot;) == &quot;none&quot; ) {
		$(&quot;#&quot; + lastRowPlusSign).css(&quot;display&quot;,&quot;&quot;);
	}	
	
}








function doSelectUser(fieldName) {
	var		url;
	var		w;
	url = '/livelink/llisapi.dll?func=user.SelectUserDlg&amp;formname=myForm&amp;fieldprefix=' + fieldName + '&amp;title=Select%20User&amp;DisplayUserName&amp;NoGroups=FALSE';
	url = url + '&amp;NoGroupsSelectable=TRUE' 
		
	w = window.open(url, &quot;&quot;, &quot;height=340,width=680,scrollbars=yes,resizable=yes,menubar=no,toolbar=yes,status=yes&quot;);		

	if ( w.focus )
		w.focus();
}






$(document).ready(function () {


    
    setAgreementTypeList();



    
    setCompanyList('ddCompany1');



    
    setProductList('ddProduct1_1');


setupniaidvars(&quot;/livelink/llisapi.dll&quot;,&quot;?func=ll&amp;objId=19844984&amp;objAction=RunReport&quot;);
$(&quot;#_1_1_5_1_Name&quot;).niaidcsuserpicker();	
$(&quot;#_1_1_6_1_Name&quot;).niaidcsuserpicker();	

});





    
        
            
                
                
                    Agreement Information
                
                
                
                    
                    
                    
                        
                        
                        
                            Action Type: 
                            
                                
                                    New Agreement

                                      
                                    Amend Agreement

                                
							
                        
                        
                        
                        
                            
                                Agreement Type: 
                            
                            
                                
                                    
                                        - None -    
                                    Clinical Trial AgreementConfidentiality AgreementEvaluation AgreementLetter of AgreementMaterial Transfer AgreementMemorandum of UnderstandingNon-Clinical StudyQuality Agreement
                                
							
							
                            
                                Multi-Company: 
                            
                            
                                
                                    Yes

                                      
                                    No

                                                                
							
                        

                    
                    
                    
                    
                        
                            
                                
                                    
                        				
                        					Company
                        					Product Name
                        					Agreement Number
                        					 
                        				
                        				
                        				
                                        
                                        	
                                        	    - None -20 / 20 Genesystems3SBioABT Holding CompanyAchaogenAeolus Pharmaceuticals, Inc.Aimmune TherapeuticsAlentic Microscience, IncAlexion PharmaceuticalsALK-Abello A/SAllergan, PlcAllerteinAmgenAngion, Inc.Araim Pharmaceuticals, Inc.Arca Bioharma, IncArmed Forces Radiobiology Research Institute (AFRRI)Aronora, Inc.ArQuleAstellas Pharma Global Development, IncAtox BioaTyr PharmaAugmentative BioTherapeutics LLCAvaxia Biologics, Inc.Baxter Healthcare CorporationBayerBCN BiosciencesBerlexBioAegis TherapeuticsBiogenBioIncept, LLCBiological Mimetics, Inc.BioMimetixBiosyn CorporationBio-Tech PharmacalBiotheraBioTransplant Inc.Bolder BioTechnology, Inc.Bristol Myers SquibbC Sixty Inc.CalibrCalifornia Institute for Biomedical ResearchCalista Therapeutics, Inc.Canadian Institutes of Health Research (CIHR)CangeneCelldex TherapeuticsCellerant TherapeuticsCellular Dynamics, Inc.Centocor, Inc.Ceramedix Holding, LLCCeramide Therapeutics, LLCChironChrysalis BiotherapeuticsCircassia LimitedCleveland BiolabsColumbia UniversityCorbus Pharmaceuticals, Inc.Coronado BiosciencesCTI Science, IncCuraGen CorporationCurevedaCytori TherapeuticsCytoSorbents, Inc.DBV TechnologiesDeyDiaCartaDiaMedicaDOR BioPharmaDova PharmaceuticalsDow Chemical Co.Duke University Medical CenterDynavaxEd Laboratories, Inc.Edison PharmaEli Lilly and CompanyEminentEnterade, LLCEntrinsic Health SolutionsEpistemEpitek, Inc.Eukarion, IncExponential Biotherapies, IncFibrogenFirstString Research, Inc.Foresee PharmaceuticalsFujisawaFull Spectrum OmegaG1 TherapeuticsGenentechGenzymeGeorgetown UniversityGlaxo/SmithklineGreer LaboratoriesHA Cell TechnologyHemostemHollis EdenHuman GhrelinHumanetics CorporationIcahn School of Medicine at Mount SinaiIDEC PharmaceuticalsIlexImmune DesignImmuneRegenImmunex CorporationInflamaxInnate Immunotherapeutics LimitedInnovation PathwaysInverseonInvionInvivoSciences, IncJanssen Research &amp; Development, LLCJB TherapeuticsJohnson &amp; JohnsonKyowa Hakko Kirin Co., Ltd.KYOWA KIRIN PHARMACEUTICAL DEVELOPMENT, INCLawrence Berkeley National LaboratoryLignaMedLongevica Pharmaceuticals Inc.MacroGenics, IncMaxygen, IncMedestea BiotechMedical College of WisconsinMedical University of South CarolinaMedImmuneMeditor Pharmaceuticals, Ltd.Medtronic MiniMedMercia PhMetroHealth Medical CenterMillenium PharmaceuticalsMitosMotus Pharmaceuticals, Inc.Multi-Party AgreementMylanN8 Medical LLCN8 Medical, Inc.NanoPass TechnologiesNeoImmuneTech, Inc.NeostemNestle Health ScienceNeumedicinesNeurocrine BiosciencesNIDDKNippon KayakuNovartisNovaScreen Biosciences CorporationNovelosNOW FoodsNuvo Research AGOcera TherapeuticsOHSU Center for Regenerative Medicine in OregonOnconova Therapeutics, Inc.OnyxOre PharmaceuticalsOsiris Therapeutics, Inc.Ossium Health, Inc.Ovation PharmaceuticalsPaloma Pharmaceuticals, Inc.ParinGenixPennsylvania State UniversityPfizerPharmaceutical Product Development (PPD)Pharmacia Diagnostics ABPhylogix, Inc.PhysiocrinesPhysioGenix, Inc.Pinnacle Biologics, Inc.Pluristem Therapeutic Inc.Point TherapeuticsPowderJect Technologies LimitedProMetic BioSciences, Inc.Proteome SystemsRadiation Control Technologies, IncRadix TherapeuticsReata Pharmaceuticals, Inc.RegeneronRepligenRestorGenix, Inc.Rhythm Pharmaceuticals, IncRocheRx Bio, Inc.Sangstat Medical CorporationSanofi, Inc.Seattle GeneticsSEPPIC IncSloan Kettering Institute for Cancer ResearchSmithKline Beecham CorporationSoligenix, IncSologenix, IncSouthwest Research InstituteSRI  InternationalSt. Camillus MedicalSynedgenTajCo IncTargazymeTDeltaSTemple UniversityTerapioTevaThe Institutes for Pharmaceutical DiscoveryTikoMed ABTobira Therapeutics, Inc.Tricorder DiagnosticsTunitas Therapeutics, Inc.UCB Pharma S.A. BelgiumUCLAUniversity of Arkansas for Medical SciencesUniversity of California San DiegoUniversity of EmoryUniversity of FloridaUniversity of Kansas Medical CenterUniversity of North Carolina at Chapel HillUniversity of PennsylvaniaUniversity of Pittsburgh Medical CenterUniversity of RochesterUniversity of UtahUniversity of VirginiaUS Biotest, Inc.ViforViraCorVitaerisWake Forest School of MedicineWyeth PharmaceuticalsWyeth-Ayerst ResearchXomaYale Universityzz-test1zz-test2zz-test-firefox
                                        	        
                                        	
                                        	
                                        	    
                                        	        
                                        	            
                                            	            - None -3-D tissues (platform technologies)AB103Actemra (tocilizumab)Adipose-Derived Stem Cells &amp; Regenerative CellsAdvair DiskusAEOL 10150Aflunov (MF59-adjuvanted)Aflunov (MF59-unadjuvanted)Ajulemic AcidAlutard SQALXN-4100 (TPO)Amb a 1Amevive (alefacept)AMG 157 (anti-TSLP)AMG 714  (anti-IL-15 antibody)Amifostine (WR1064)Amino Acid Cocktailangiotensin peptidesAnti IL-17 mabAnti-ceramide antibodyAnti-Il12Anti-IL-5 (mepolizumab)anti-TGF-B antibodyanti-TNFAR101ARA 290Aralast NPARQ 501ARQ 900ASMase pathway in GI tractAssay Reagent KitsAST-120AtorvastatinAvonexBAC-3BaminerceptBB3BCN512Belatacept (LEA29Y)BelimumabBIO300BMS-188667BMS-224818BMS-224819Bone Marrow bankBPIBrentuximab (Adcentris)C22TTC2E5Ca-DTPACampath-1HCaptoprilCarfilzomibCat allergen extractsCat DanderCat-PADCat-PAD &amp; placebo for Cat-PADCBLB-502CBLB-612CD 34 Reagent KitsCD3+/CD19+ depleted bone marrowCDK4/6 InhibitorsCDX-301CenicrivirocCeragenins (Cationic Steroid Antibiotics)CeramideCG53135-05 (FGF-20)Chalcone derivative (2-trifluoromethyl-2'-methoxychalone)Characterized Peanut AllergenChemetCLT008Cockroach Allergen Extractcongener compoundsCTLA4-IgG4mDeltaGDeoxyspergualinDIM (3DM99DupilumabEA-230ECIS Based Automated Stem cell functionality AssayEDL2000EFK2 inhibitorEMP-123EnalaprilEnterade-REntericSorbEpi-1AEPI-743EpiPenEpiPen Jr.Epix-107Epix-111Epix-112ErythropoietinEtanerceptEUK-189Fab59FG-4539Fibroblast Growth Factor peptide (FGF-P)Flaxseed (Z6748)Flonase Nasal SprayFlovent DiskusFlt3 Receptor-Interacting Lectin (FRIL)GenisteinGlucopyranosyl Lipid A (GLA)Glycerinated Peanut Allergenic ExtractGranexin GelGrazaxGTS-21H1N1 (Influenza A) 2009 S-01v VaccineH5N1 Flu AdjuvantH5N1 Flu VaccineHemp OilhOKT3 (anti-CD3)HOPO derivativesHouse dust mite- mixHuman iPS cell manufacturing platformhuman recombinant thrombopoietinhyaluronic acidIDEC-131 (anti-CD154)IL-12Immune Globulin IntravenousImmunostimnlary oligodeoxyribonucleotide conjugate (AIC)Imprime WGPInfliximabIntradermal Fcy1-Fel d1 fusion protein (GFD)IPW5371IsletIsolex 300i Disposable SetsIsolex 300i Magneticcell Selection System MachineIsothiouronium derivativesJBT-101Keyhole Limpet Hemocyanin (KLH)Leucotropin (GM-CSF)LeukineLHRH-AntLovastatin TabletsLow Molecular Weight Dextran SulfateLulizumabMAS-1Maxy-G34MC8  insertsMEDI-507Mesenchymal Stem CellsMGA-031MicronJet600 needlesMIS416moleculesMontanide ISA 51 VGMontanide ISA 51 VG Sterilemouse Myeloid Progenitor Cells (mMPc)MozobilMultiPepMultiStemN/ANadololNBI-5788Neulasta (pegfilgrastim)Neumune (androst-5-ene-3bNeupogenNilotinibNon-Fat Dry Milk (NFD Milk)NOV-002NplateNRF2 upregulatorsNU206 (R-Spondin 1)Nucala (mepolizumab)ODSHOligonucleotide morpholino to CD47ON 01210.Na (Ex-Rad)Oral DTPAOrencia (abatacept)Otelixizumabotelixizumab and sirukumabP529PAAGPBI-1402Peanut Protein Oral Immunotherapy (AR101)PEGylation IgG fusionphosphonolPhysiocrinesplacebo for Abataceptplacebo for AMG 157placebo for Aralast NPplacebo for Atorvastatinplacebo for Cat-PADplacebo for Cockroach Allergen Extractplacebo for Flovent Diskusplacebo for Nadololplacebo for Rituxanplacebo for Viaskin Peanutplacebo for XolairPLacental eXpanded (PLX-RAD) cellsPleiotropin (PTN)Plerixafor (AMD3100)PoC BiodosimeterPPARgamma ligandsPre-Implantation Factor (PIF)PREMEAPrograf (tacrolimus)Proleukin (IL-2)RadilexRagweed Allergen Immunostimulatory Sequence ConjugateRapamune (sirolimus)rBPI-21Regulatory T cells (Treg)Relamorelin (RM-131)RG2077 (CTLA4-IG)Rituxan (rituximab)RLIP76RM-131RN168RWJ-800088Rx100Rx101Secoisolariciresinaol diglucosides (SDGs)Secukinumab (anti-IL-17)SGX202 (oral beclomethasone dipropionate or BDP)SGX942SphingosineStandardized Cat Hair ExtractstatinsSynexan (SP001)synthetic triterpenoids RTA 401synthetic triterpenoids RTA 402TabalostatTBO-filgrastimT-cell epitode peptidesTempolTeplizumabThymoglobulinTimothy grassTP508TPOmTrichuris suis ova (TSO)TXA127 (formulated angiotensin 1-7)Unadjuvanted H1N1 Influenza Virus VaccineVelaferminVelcade (bortezomib)Ventolin HFA (albuterol sulfate)Very Small Embryonic-Like cells (VSELs)Viaskin PeanutViral Panel (human samples)Vitamin D3WF10Xolair (omalizumab)XOMA 052Yel 002Yel/Rad 1/2Yel/Rad 1/2 derivativesZenapax (daclizumab)Zn-DTPAZortress (Everolimus)
                                            	                
                                                        
                                                        
                                                                                                                        
                                                        
                                                    
                                                
                                        	
                                        	
                                        	    T.19.38.0001
                                        	        
                                        	
                                            
                                                
    							                                                    	
                                             - None -20 / 20 Genesystems3SBioABT Holding CompanyAchaogenAeolus Pharmaceuticals, Inc.Aimmune TherapeuticsAlentic Microscience, IncAlexion PharmaceuticalsALK-Abello A/SAllergan, PlcAllerteinAmgenAngion, Inc.Araim Pharmaceuticals, Inc.Arca Bioharma, IncArmed Forces Radiobiology Research Institute (AFRRI)Aronora, Inc.ArQuleAstellas Pharma Global Development, IncAtox BioaTyr PharmaAugmentative BioTherapeutics LLCAvaxia Biologics, Inc.Baxter Healthcare CorporationBayerBCN BiosciencesBerlexBioAegis TherapeuticsBiogenBioIncept, LLCBiological Mimetics, Inc.BioMimetixBiosyn CorporationBio-Tech PharmacalBiotheraBioTransplant Inc.Bolder BioTechnology, Inc.Bristol Myers SquibbC Sixty Inc.CalibrCalifornia Institute for Biomedical ResearchCalista Therapeutics, Inc.Canadian Institutes of Health Research (CIHR)CangeneCelldex TherapeuticsCellerant TherapeuticsCellular Dynamics, Inc.Centocor, Inc.Ceramedix Holding, LLCCeramide Therapeutics, LLCChironChrysalis BiotherapeuticsCircassia LimitedCleveland BiolabsColumbia UniversityCorbus Pharmaceuticals, Inc.Coronado BiosciencesCTI Science, IncCuraGen CorporationCurevedaCytori TherapeuticsCytoSorbents, Inc.DBV TechnologiesDeyDiaCartaDiaMedicaDOR BioPharmaDova PharmaceuticalsDow Chemical Co.Duke University Medical CenterDynavaxEd Laboratories, Inc.Edison PharmaEli Lilly and CompanyEminentEnterade, LLCEntrinsic Health SolutionsEpistemEpitek, Inc.Eukarion, IncExponential Biotherapies, IncFibrogenFirstString Research, Inc.Foresee PharmaceuticalsFujisawaFull Spectrum OmegaG1 TherapeuticsGenentechGenzymeGeorgetown UniversityGlaxo/SmithklineGreer LaboratoriesHA Cell TechnologyHemostemHollis EdenHuman GhrelinHumanetics CorporationIcahn School of Medicine at Mount SinaiIDEC PharmaceuticalsIlexImmune DesignImmuneRegenImmunex CorporationInflamaxInnate Immunotherapeutics LimitedInnovation PathwaysInverseonInvionInvivoSciences, IncJanssen Research &amp; Development, LLCJB TherapeuticsJohnson &amp; JohnsonKyowa Hakko Kirin Co., Ltd.KYOWA KIRIN PHARMACEUTICAL DEVELOPMENT, INCLawrence Berkeley National LaboratoryLignaMedLongevica Pharmaceuticals Inc.MacroGenics, IncMaxygen, IncMedestea BiotechMedical College of WisconsinMedical University of South CarolinaMedImmuneMeditor Pharmaceuticals, Ltd.Medtronic MiniMedMercia PhMetroHealth Medical CenterMillenium PharmaceuticalsMitosMotus Pharmaceuticals, Inc.Multi-Party AgreementMylanN8 Medical LLCN8 Medical, Inc.NanoPass TechnologiesNeoImmuneTech, Inc.NeostemNestle Health ScienceNeumedicinesNeurocrine BiosciencesNIDDKNippon KayakuNovartisNovaScreen Biosciences CorporationNovelosNOW FoodsNuvo Research AGOcera TherapeuticsOHSU Center for Regenerative Medicine in OregonOnconova Therapeutics, Inc.OnyxOre PharmaceuticalsOsiris Therapeutics, Inc.Ossium Health, Inc.Ovation PharmaceuticalsPaloma Pharmaceuticals, Inc.ParinGenixPennsylvania State UniversityPfizerPharmaceutical Product Development (PPD)Pharmacia Diagnostics ABPhylogix, Inc.PhysiocrinesPhysioGenix, Inc.Pinnacle Biologics, Inc.Pluristem Therapeutic Inc.Point TherapeuticsPowderJect Technologies LimitedProMetic BioSciences, Inc.Proteome SystemsRadiation Control Technologies, IncRadix TherapeuticsReata Pharmaceuticals, Inc.RegeneronRepligenRestorGenix, Inc.Rhythm Pharmaceuticals, IncRocheRx Bio, Inc.Sangstat Medical CorporationSanofi, Inc.Seattle GeneticsSEPPIC IncSloan Kettering Institute for Cancer ResearchSmithKline Beecham CorporationSoligenix, IncSologenix, IncSouthwest Research InstituteSRI  InternationalSt. Camillus MedicalSynedgenTajCo IncTargazymeTDeltaSTemple UniversityTerapioTevaThe Institutes for Pharmaceutical DiscoveryTikoMed ABTobira Therapeutics, Inc.Tricorder DiagnosticsTunitas Therapeutics, Inc.UCB Pharma S.A. BelgiumUCLAUniversity of Arkansas for Medical SciencesUniversity of California San DiegoUniversity of EmoryUniversity of FloridaUniversity of Kansas Medical CenterUniversity of North Carolina at Chapel HillUniversity of PennsylvaniaUniversity of Pittsburgh Medical CenterUniversity of RochesterUniversity of UtahUniversity of VirginiaUS Biotest, Inc.ViforViraCorVitaerisWake Forest School of MedicineWyeth PharmaceuticalsWyeth-Ayerst ResearchXomaYale Universityzz-test1zz-test2zz-test-firefox     	    	        	                	            - None -3-D tissues (platform technologies)AB103Actemra (tocilizumab)Adipose-Derived Stem Cells &amp; Regenerative CellsAdvair DiskusAEOL 10150Aflunov (MF59-adjuvanted)Aflunov (MF59-unadjuvanted)Ajulemic AcidAlutard SQALXN-4100 (TPO)Amb a 1Amevive (alefacept)AMG 157 (anti-TSLP)AMG 714  (anti-IL-15 antibody)Amifostine (WR1064)Amino Acid Cocktailangiotensin peptidesAnti IL-17 mabAnti-ceramide antibodyAnti-Il12Anti-IL-5 (mepolizumab)anti-TGF-B antibodyanti-TNFAR101ARA 290Aralast NPARQ 501ARQ 900ASMase pathway in GI tractAssay Reagent KitsAST-120AtorvastatinAvonexBAC-3BaminerceptBB3BCN512Belatacept (LEA29Y)BelimumabBIO300BMS-188667BMS-224818BMS-224819Bone Marrow bankBPIBrentuximab (Adcentris)C22TTC2E5Ca-DTPACampath-1HCaptoprilCarfilzomibCat allergen extractsCat DanderCat-PADCat-PAD &amp; placebo for Cat-PADCBLB-502CBLB-612CD 34 Reagent KitsCD3+/CD19+ depleted bone marrowCDK4/6 InhibitorsCDX-301CenicrivirocCeragenins (Cationic Steroid Antibiotics)CeramideCG53135-05 (FGF-20)Chalcone derivative (2-trifluoromethyl-2'-methoxychalone)Characterized Peanut AllergenChemetCLT008Cockroach Allergen Extractcongener compoundsCTLA4-IgG4mDeltaGDeoxyspergualinDIM (3DM99DupilumabEA-230ECIS Based Automated Stem cell functionality AssayEDL2000EFK2 inhibitorEMP-123EnalaprilEnterade-REntericSorbEpi-1AEPI-743EpiPenEpiPen Jr.Epix-107Epix-111Epix-112ErythropoietinEtanerceptEUK-189Fab59FG-4539Fibroblast Growth Factor peptide (FGF-P)Flaxseed (Z6748)Flonase Nasal SprayFlovent DiskusFlt3 Receptor-Interacting Lectin (FRIL)GenisteinGlucopyranosyl Lipid A (GLA)Glycerinated Peanut Allergenic ExtractGranexin GelGrazaxGTS-21H1N1 (Influenza A) 2009 S-01v VaccineH5N1 Flu AdjuvantH5N1 Flu VaccineHemp OilhOKT3 (anti-CD3)HOPO derivativesHouse dust mite- mixHuman iPS cell manufacturing platformhuman recombinant thrombopoietinhyaluronic acidIDEC-131 (anti-CD154)IL-12Immune Globulin IntravenousImmunostimnlary oligodeoxyribonucleotide conjugate (AIC)Imprime WGPInfliximabIntradermal Fcy1-Fel d1 fusion protein (GFD)IPW5371IsletIsolex 300i Disposable SetsIsolex 300i Magneticcell Selection System MachineIsothiouronium derivativesJBT-101Keyhole Limpet Hemocyanin (KLH)Leucotropin (GM-CSF)LeukineLHRH-AntLovastatin TabletsLow Molecular Weight Dextran SulfateLulizumabMAS-1Maxy-G34MC8  insertsMEDI-507Mesenchymal Stem CellsMGA-031MicronJet600 needlesMIS416moleculesMontanide ISA 51 VGMontanide ISA 51 VG Sterilemouse Myeloid Progenitor Cells (mMPc)MozobilMultiPepMultiStemN/ANadololNBI-5788Neulasta (pegfilgrastim)Neumune (androst-5-ene-3bNeupogenNilotinibNon-Fat Dry Milk (NFD Milk)NOV-002NplateNRF2 upregulatorsNU206 (R-Spondin 1)Nucala (mepolizumab)ODSHOligonucleotide morpholino to CD47ON 01210.Na (Ex-Rad)Oral DTPAOrencia (abatacept)Otelixizumabotelixizumab and sirukumabP529PAAGPBI-1402Peanut Protein Oral Immunotherapy (AR101)PEGylation IgG fusionphosphonolPhysiocrinesplacebo for Abataceptplacebo for AMG 157placebo for Aralast NPplacebo for Atorvastatinplacebo for Cat-PADplacebo for Cockroach Allergen Extractplacebo for Flovent Diskusplacebo for Nadololplacebo for Rituxanplacebo for Viaskin Peanutplacebo for XolairPLacental eXpanded (PLX-RAD) cellsPleiotropin (PTN)Plerixafor (AMD3100)PoC BiodosimeterPPARgamma ligandsPre-Implantation Factor (PIF)PREMEAPrograf (tacrolimus)Proleukin (IL-2)RadilexRagweed Allergen Immunostimulatory Sequence ConjugateRapamune (sirolimus)rBPI-21Regulatory T cells (Treg)Relamorelin (RM-131)RG2077 (CTLA4-IG)Rituxan (rituximab)RLIP76RM-131RN168RWJ-800088Rx100Rx101Secoisolariciresinaol diglucosides (SDGs)Secukinumab (anti-IL-17)SGX202 (oral beclomethasone dipropionate or BDP)SGX942SphingosineStandardized Cat Hair ExtractstatinsSynexan (SP001)synthetic triterpenoids RTA 401synthetic triterpenoids RTA 402TabalostatTBO-filgrastimT-cell epitode peptidesTempolTeplizumabThymoglobulinTimothy grassTP508TPOmTrichuris suis ova (TSO)TXA127 (formulated angiotensin 1-7)Unadjuvanted H1N1 Influenza Virus VaccineVelaferminVelcade (bortezomib)Ventolin HFA (albuterol sulfate)Very Small Embryonic-Like cells (VSELs)Viaskin PeanutViral Panel (human samples)Vitamin D3WF10Xolair (omalizumab)XOMA 052Yel 002Yel/Rad 1/2Yel/Rad 1/2 derivativesZenapax (daclizumab)Zn-DTPAZortress (Everolimus)   	                                                                                      	    - None -3-D tissues (platform technologies)AB103Actemra (tocilizumab)Adipose-Derived Stem Cells &amp; Regenerative CellsAdvair DiskusAEOL 10150Aflunov (MF59-adjuvanted)Aflunov (MF59-unadjuvanted)Ajulemic AcidAlutard SQALXN-4100 (TPO)Amb a 1Amevive (alefacept)AMG 157 (anti-TSLP)AMG 714  (anti-IL-15 antibody)Amifostine (WR1064)Amino Acid Cocktailangiotensin peptidesAnti IL-17 mabAnti-ceramide antibodyAnti-Il12Anti-IL-5 (mepolizumab)anti-TGF-B antibodyanti-TNFAR101ARA 290Aralast NPARQ 501ARQ 900ASMase pathway in GI tractAssay Reagent KitsAST-120AtorvastatinAvonexBAC-3BaminerceptBB3BCN512Belatacept (LEA29Y)BelimumabBIO300BMS-188667BMS-224818BMS-224819Bone Marrow bankBPIBrentuximab (Adcentris)C22TTC2E5Ca-DTPACampath-1HCaptoprilCarfilzomibCat allergen extractsCat DanderCat-PADCat-PAD &amp; placebo for Cat-PADCBLB-502CBLB-612CD 34 Reagent KitsCD3+/CD19+ depleted bone marrowCDK4/6 InhibitorsCDX-301CenicrivirocCeragenins (Cationic Steroid Antibiotics)CeramideCG53135-05 (FGF-20)Chalcone derivative (2-trifluoromethyl-2'-methoxychalone)Characterized Peanut AllergenChemetCLT008Cockroach Allergen Extractcongener compoundsCTLA4-IgG4mDeltaGDeoxyspergualinDIM (3DM99DupilumabEA-230ECIS Based Automated Stem cell functionality AssayEDL2000EFK2 inhibitorEMP-123EnalaprilEnterade-REntericSorbEpi-1AEPI-743EpiPenEpiPen Jr.Epix-107Epix-111Epix-112ErythropoietinEtanerceptEUK-189Fab59FG-4539Fibroblast Growth Factor peptide (FGF-P)Flaxseed (Z6748)Flonase Nasal SprayFlovent DiskusFlt3 Receptor-Interacting Lectin (FRIL)GenisteinGlucopyranosyl Lipid A (GLA)Glycerinated Peanut Allergenic ExtractGranexin GelGrazaxGTS-21H1N1 (Influenza A) 2009 S-01v VaccineH5N1 Flu AdjuvantH5N1 Flu VaccineHemp OilhOKT3 (anti-CD3)HOPO derivativesHouse dust mite- mixHuman iPS cell manufacturing platformhuman recombinant thrombopoietinhyaluronic acidIDEC-131 (anti-CD154)IL-12Immune Globulin IntravenousImmunostimnlary oligodeoxyribonucleotide conjugate (AIC)Imprime WGPInfliximabIntradermal Fcy1-Fel d1 fusion protein (GFD)IPW5371IsletIsolex 300i Disposable SetsIsolex 300i Magneticcell Selection System MachineIsothiouronium derivativesJBT-101Keyhole Limpet Hemocyanin (KLH)Leucotropin (GM-CSF)LeukineLHRH-AntLovastatin TabletsLow Molecular Weight Dextran SulfateLulizumabMAS-1Maxy-G34MC8  insertsMEDI-507Mesenchymal Stem CellsMGA-031MicronJet600 needlesMIS416moleculesMontanide ISA 51 VGMontanide ISA 51 VG Sterilemouse Myeloid Progenitor Cells (mMPc)MozobilMultiPepMultiStemN/ANadololNBI-5788Neulasta (pegfilgrastim)Neumune (androst-5-ene-3bNeupogenNilotinibNon-Fat Dry Milk (NFD Milk)NOV-002NplateNRF2 upregulatorsNU206 (R-Spondin 1)Nucala (mepolizumab)ODSHOligonucleotide morpholino to CD47ON 01210.Na (Ex-Rad)Oral DTPAOrencia (abatacept)Otelixizumabotelixizumab and sirukumabP529PAAGPBI-1402Peanut Protein Oral Immunotherapy (AR101)PEGylation IgG fusionphosphonolPhysiocrinesplacebo for Abataceptplacebo for AMG 157placebo for Aralast NPplacebo for Atorvastatinplacebo for Cat-PADplacebo for Cockroach Allergen Extractplacebo for Flovent Diskusplacebo for Nadololplacebo for Rituxanplacebo for Viaskin Peanutplacebo for XolairPLacental eXpanded (PLX-RAD) cellsPleiotropin (PTN)Plerixafor (AMD3100)PoC BiodosimeterPPARgamma ligandsPre-Implantation Factor (PIF)PREMEAPrograf (tacrolimus)Proleukin (IL-2)RadilexRagweed Allergen Immunostimulatory Sequence ConjugateRapamune (sirolimus)rBPI-21Regulatory T cells (Treg)Relamorelin (RM-131)RG2077 (CTLA4-IG)Rituxan (rituximab)RLIP76RM-131RN168RWJ-800088Rx100Rx101Secoisolariciresinaol diglucosides (SDGs)Secukinumab (anti-IL-17)SGX202 (oral beclomethasone dipropionate or BDP)SGX942SphingosineStandardized Cat Hair ExtractstatinsSynexan (SP001)synthetic triterpenoids RTA 401synthetic triterpenoids RTA 402TabalostatTBO-filgrastimT-cell epitode peptidesTempolTeplizumabThymoglobulinTimothy grassTP508TPOmTrichuris suis ova (TSO)TXA127 (formulated angiotensin 1-7)Unadjuvanted H1N1 Influenza Virus VaccineVelaferminVelcade (bortezomib)Ventolin HFA (albuterol sulfate)Very Small Embryonic-Like cells (VSELs)Viaskin PeanutViral Panel (human samples)Vitamin D3WF10Xolair (omalizumab)XOMA 052Yel 002Yel/Rad 1/2Yel/Rad 1/2 derivativesZenapax (daclizumab)Zn-DTPAZortress (Everolimus)                                                   	    - None -3-D tissues (platform technologies)AB103Actemra (tocilizumab)Adipose-Derived Stem Cells &amp; Regenerative CellsAdvair DiskusAEOL 10150Aflunov (MF59-adjuvanted)Aflunov (MF59-unadjuvanted)Ajulemic AcidAlutard SQALXN-4100 (TPO)Amb a 1Amevive (alefacept)AMG 157 (anti-TSLP)AMG 714  (anti-IL-15 antibody)Amifostine (WR1064)Amino Acid Cocktailangiotensin peptidesAnti IL-17 mabAnti-ceramide antibodyAnti-Il12Anti-IL-5 (mepolizumab)anti-TGF-B antibodyanti-TNFAR101ARA 290Aralast NPARQ 501ARQ 900ASMase pathway in GI tractAssay Reagent KitsAST-120AtorvastatinAvonexBAC-3BaminerceptBB3BCN512Belatacept (LEA29Y)BelimumabBIO300BMS-188667BMS-224818BMS-224819Bone Marrow bankBPIBrentuximab (Adcentris)C22TTC2E5Ca-DTPACampath-1HCaptoprilCarfilzomibCat allergen extractsCat DanderCat-PADCat-PAD &amp; placebo for Cat-PADCBLB-502CBLB-612CD 34 Reagent KitsCD3+/CD19+ depleted bone marrowCDK4/6 InhibitorsCDX-301CenicrivirocCeragenins (Cationic Steroid Antibiotics)CeramideCG53135-05 (FGF-20)Chalcone derivative (2-trifluoromethyl-2'-methoxychalone)Characterized Peanut AllergenChemetCLT008Cockroach Allergen Extractcongener compoundsCTLA4-IgG4mDeltaGDeoxyspergualinDIM (3DM99DupilumabEA-230ECIS Based Automated Stem cell functionality AssayEDL2000EFK2 inhibitorEMP-123EnalaprilEnterade-REntericSorbEpi-1AEPI-743EpiPenEpiPen Jr.Epix-107Epix-111Epix-112ErythropoietinEtanerceptEUK-189Fab59FG-4539Fibroblast Growth Factor peptide (FGF-P)Flaxseed (Z6748)Flonase Nasal SprayFlovent DiskusFlt3 Receptor-Interacting Lectin (FRIL)GenisteinGlucopyranosyl Lipid A (GLA)Glycerinated Peanut Allergenic ExtractGranexin GelGrazaxGTS-21H1N1 (Influenza A) 2009 S-01v VaccineH5N1 Flu AdjuvantH5N1 Flu VaccineHemp OilhOKT3 (anti-CD3)HOPO derivativesHouse dust mite- mixHuman iPS cell manufacturing platformhuman recombinant thrombopoietinhyaluronic acidIDEC-131 (anti-CD154)IL-12Immune Globulin IntravenousImmunostimnlary oligodeoxyribonucleotide conjugate (AIC)Imprime WGPInfliximabIntradermal Fcy1-Fel d1 fusion protein (GFD)IPW5371IsletIsolex 300i Disposable SetsIsolex 300i Magneticcell Selection System MachineIsothiouronium derivativesJBT-101Keyhole Limpet Hemocyanin (KLH)Leucotropin (GM-CSF)LeukineLHRH-AntLovastatin TabletsLow Molecular Weight Dextran SulfateLulizumabMAS-1Maxy-G34MC8  insertsMEDI-507Mesenchymal Stem CellsMGA-031MicronJet600 needlesMIS416moleculesMontanide ISA 51 VGMontanide ISA 51 VG Sterilemouse Myeloid Progenitor Cells (mMPc)MozobilMultiPepMultiStemN/ANadololNBI-5788Neulasta (pegfilgrastim)Neumune (androst-5-ene-3bNeupogenNilotinibNon-Fat Dry Milk (NFD Milk)NOV-002NplateNRF2 upregulatorsNU206 (R-Spondin 1)Nucala (mepolizumab)ODSHOligonucleotide morpholino to CD47ON 01210.Na (Ex-Rad)Oral DTPAOrencia (abatacept)Otelixizumabotelixizumab and sirukumabP529PAAGPBI-1402Peanut Protein Oral Immunotherapy (AR101)PEGylation IgG fusionphosphonolPhysiocrinesplacebo for Abataceptplacebo for AMG 157placebo for Aralast NPplacebo for Atorvastatinplacebo for Cat-PADplacebo for Cockroach Allergen Extractplacebo for Flovent Diskusplacebo for Nadololplacebo for Rituxanplacebo for Viaskin Peanutplacebo for XolairPLacental eXpanded (PLX-RAD) cellsPleiotropin (PTN)Plerixafor (AMD3100)PoC BiodosimeterPPARgamma ligandsPre-Implantation Factor (PIF)PREMEAPrograf (tacrolimus)Proleukin (IL-2)RadilexRagweed Allergen Immunostimulatory Sequence ConjugateRapamune (sirolimus)rBPI-21Regulatory T cells (Treg)Relamorelin (RM-131)RG2077 (CTLA4-IG)Rituxan (rituximab)RLIP76RM-131RN168RWJ-800088Rx100Rx101Secoisolariciresinaol diglucosides (SDGs)Secukinumab (anti-IL-17)SGX202 (oral beclomethasone dipropionate or BDP)SGX942SphingosineStandardized Cat Hair ExtractstatinsSynexan (SP001)synthetic triterpenoids RTA 401synthetic triterpenoids RTA 402TabalostatTBO-filgrastimT-cell epitode peptidesTempolTeplizumabThymoglobulinTimothy grassTP508TPOmTrichuris suis ova (TSO)TXA127 (formulated angiotensin 1-7)Unadjuvanted H1N1 Influenza Virus VaccineVelaferminVelcade (bortezomib)Ventolin HFA (albuterol sulfate)Very Small Embryonic-Like cells (VSELs)Viaskin PeanutViral Panel (human samples)Vitamin D3WF10Xolair (omalizumab)XOMA 052Yel 002Yel/Rad 1/2Yel/Rad 1/2 derivativesZenapax (daclizumab)Zn-DTPAZortress (Everolimus)                                                    T.19.27.0010               
                                         
                                        
                        			
                        		
                        	
                        
                    
                    
                    
                           
                           
                    
                    
                    
                    
                        
                        
                            
                                Protocol Number: 
                            
                            
                                
                                    
                                
							
                            
                                Network Name: 
                            
                            
                                
                                    
                                
							
                        

                        
                            
                                Regulatory Officer: 
                            
                            
                                
                                
                                    
                                    
                                    
                                                                         
                                
							
                            
                                Medical Officer: 
                            
                            
                                
                                
                                    
                                    
                                    
                                                                          
                                
							
                        

                    
                                        
                    
                    
                    
                
                
                
            
            
        
    


					
					

::-webkit-input-placeholder { /* Chrome/Opera/Safari */
  font-style:italic;
}
::-moz-placeholder { /* Firefox 19+ */
  font-style:italic;
}
:-ms-input-placeholder { /* IE 10+ */
  font-style:italic;
}
:-moz-placeholder { /* Firefox 18- */
  font-style:italic;
}



 


    
        
            
                Comments
            
            
                
                    
                        
                            
                        
                    
                
                
                     	
                        
                         
            
        
    
	











					
				
				
				
				
				
					
					    
						 
                        
					
				
			
				
			
				



	 td.footerTable
		{
			background: #015294;
		}
	 a.footer
		{
			font-family:verdana;font-size:9px;
		}
	a.footer:link
		{
			text-decoration: none;
			color: White;        
			border: 0;
		}
	a.footer:visited
		{
			text-decoration: none;
			color: White;        
		}
	a.footer:hover
		{
			text-decoration: underline;
			color: White;
		}
	a.footer:active
		{
			text-decoration: underline;
			color: White;
		}    



    
        
            
                
                    
                        
						 
						Department of Health and Human Services
                    
                    
                        
						 
						National Institutes of Health (NIH)
                    
                    
                        
                            
						
                    										
                
            
        
    


				

			
				
				
				
				
						
				
					
			
        
                
    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/center[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Today is'])[1]/following::center[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//center</value>
   </webElementXpaths>
</WebElementEntity>
