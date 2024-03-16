use {
    nfm_core::Parser,
    criterion::{ criterion_group, criterion_main, Criterion, },
};

const MARKDOWN: &'static str = r#"
# Frequently Asked Questions by the Pharmaceutical Industry in Preparing for the U.S. DSCSA

Notes on [this document](pdfs/cardinal-health-pharma-dscsa-faq.pdf).

## GS1

- GS1
    - A neutral, not-for-profit global organization that develops and maintains the most widely-used supply chain standards system in the world.
- GS1 US
    - A member of GS1 global.
    - Is a not-for-profit information standards organization that facilitates industry collaboration to help improve supply chain visibility and efficiency through the use of GS1 standards.
- GS1 Healthcare
    - A global, voluntary healthcare user group developing global standards for the healthcare supply chain and advancing global harmonization.
- GS1 Healthcare US
    - An industry group that focuses on driving the adoption and implementation of GS1 Standards in the healthcare industry in the United States to improve patient safety and supply chain efficiency.

## IDENTIFY: GS1 Identification Numbers

### Global Trade Item Number (_GTIN_)

#### <what-is-a-gtin>What is a _GTIN_?

- A globally unique GS1 identification number used to identify [_**trade items**_](#trade-items)
- Assigned by the brand owner of the product and used to identify products as they move through the global supply chain to the hospital or consumer/patient.
- Used to identify a product at each [_**packaging level**_](#packaging-level)

#### National Drug Code (_NDC_)

- A 10-digit identification number established by the US FDA to identify drugs in accordance with Section 510 of the federal food, drug, and cosmetic act
- Consists of 3 segments which specify the drug product's labeler, trade product, and package size
- Each NDC uniquely identifies a specific drug having a particular dosage form, strength, and package size

#### Differences between _GTIN_ and _NDC_

- The _GTIN_ is used to identify all types of **trade items**, including drugs, medical devices, consumer products outside of the healthcare sector, and non-consumer **trade items** across all sectors
- The _NDC_ is only used to identify drugs, and only drugs subject to US FDA regulation
- The _GTIN_ uniquely identifies not only the individual sale unit of packaging, but also higher level groupings of [_**homogeneous packaging**_](#homogeneous-packaging) such as homogeneous cases, homomgeneous pallets, etc. The NDC only identifies the drug itself and does not distinguish between the individual sale and higher level groupings

#### Relationship between an _NDC_ and a _GTIN_

- Drug **trade items** that have an _NDC_ can be given a _GTIN_ that embeds the _NDC_
    - This is the best practice
- For an individual sale unit, the _NDC_ is embedded in a [_GTIN-12_](#gtin-12) by combining the following elements in left-to-right order:
    - The digit "3"
    - The 10-digit _NDC_
    - A check digit, computed from the previous 11 digits according to the GS1
        - [Check digit calculator](https://www.gs1us.org/tools/check-digit-calculator)
- In some situations the GTIN-12 has to be expressed in 14-digit format, which is done by adding two zero digits to the left of the _GTIN-12_
    - When storing in a database, the 14-digit format should always be used
- For a higher-level grouping such as a _homogeneous case_, _homogeneous pallet_, etc, the _NDC_ is embedded in a [_GTIN-14_](#gtin-14) by combining the following elements in left-to-right order:
    - The digit between 1 and 8 (inclusive) chosen differently for each grouping
        - This is called the "_indicator digit_"
        - The value of the indicator digit does not matter so long as a different indicator digit is used for each configuration
    - The two digits "03"
    - The 10-digit _NDC_
    - A check digit, computed from the previous 13 digits according to the aforementioned check digit calculator

#### Digits in a _GTIN_

- A _GTIN_ assigned for an individual sale unit has 12 digits and is called a _GTIN-12_
    - This allows it to be carried in a _UPC-A_ barcode
- Higher level groupings will have 14 digits and is called a _GTIN-14_
- _GTIN-13_ and _GTIN-8_ are also valid formats, but never used for US pharmaceuticals that have an _NDC_

#### Misc _GTIN_ Info

- If a case quantity is changed, then a new _GTIN_ is required
- GTINs are not registered with the FDA

#### Serialized GTIN (_SGTIN_)

- An _SGTIN_ is the combination of a _GTIN_ and a unqiue serial number of up to 20 alphanumeric characters
- Each instance of a given _**trade item**_ receives a different [_**serial number**_](#serial-number)

#### Standardized Numerical Identification (_SNI_)

- _SNI_ is the FDA's term for the unique identification mandated by the DSCSA
- The FDA provides guidance for the _SNI_ for package-level identification of prescription drugs
    - It specifies that an SNI should include the _NDC_ for the drug and a unqiue serial number of up to 20 alphanumeric characters generated by the manufacturer or repackager
    - An _SGTIN_ can be used as an _SNI_

#### _SGTIN_ and _SNI_

- The _SGTIN_ is compatible with the _SNI_ requirements defined by the FDA

### Serial Shipping Container Code (_SSCC_)

#### What is a _SSCC_?

- A globally unique GS1 identification number used to identify individual [_**logistical units**_](#logistical-units)

#### Generation of _SSCC_s

- The shipping party creates the _SSCC_
- When building a shipment for the buyer, The shipping party creates an SSCC using the _Shipper's GS1 Company Prefix_ and places a logistics label containing the _SSCC_ on the [_**shipment unit**_](#shipment-unit)
- Suppliers are responsible for assigning (_allocating_) _SSCC_s to their logistics units

#### How is an _SSCC_ Generated?

- A string of 18 digits that is globally unique
- Generated by combining the following four components in left-to-right order:
    - **Extension Digit**: A single digit between 0 and 9 (_inclusive_)
        - Available to increase the capacity of the _Serial Reference_
        - Has no other logic or meaning
        - Shippers who do not need the additional capacity may simply use "0" (or any other fixed value)
    - **GS1 Company Prefix**: A globally unique number assigned by GS1 US to the company or organization who generates the _SSCC_
        - Between 6 and 11 digits
        - The shorter the company prefix, the longer the serial reference
        - The shorter the company prefix, the greater number of _SSCC_s that may be allocated
        - The same company prefix can be used to generate _GTIN_s and _SSCC_s
    - **Serial Reference**: A number assigned by the holder of the **GS1 Company Prefix** to uniquely identify a **logistic unit**
    - Together with the extension digit, is the "_serial_" part that is assigned one-by-one by the company to create a globally unique _SSCC_ for each logistic unit
    - The number of digits in the **Serial Reference** varies in length depending on the length of the **GS1 Company Prefix** so that the total number of digits in the _SSCC_ is 18
    - **Check Digit**: A single digit computed from the previous 17 digits according to the aforementioned check-digit-calulator

#### How is an **SSCC** different than a **GTIN**?

- _SSCC_s are distinctly different from _GTIN_s
- The _SSCC_ acts as a license plate to track a shipment of logistics units through the supply chain
- The _GTIN_ uniquely identifies **trade items** (products and services)

#### How many **SSCC**s can I create?

|=$ GS1 Company Prefix Digits |=$ Serial Reference Digits |=$ Number of **SSCC**s (no ext digit) |=$ Number of **SSCC**s (ext digit)
|$                          6 |$                       10 |$                      10,000,000,000 |$                  100,000,000,000
|$                          7 |$                        9 |$                       1,000,000,000 |$                   10,000,000,000
|$                          8 |$                        8 |$                         100,000,000 |$                    1,000,000,000
|$                          9 |$                        7 |$                          10,000,000 |$                      100,000,000
|$                         10 |$                        6 |$                           1,000,000 |$                       10,000,000
|$                         11 |$                        5 |$                             100,000 |$                        1,000,000

#### Can the Extension Digit Be Used to Differentiate _SSCC_s by Packaging Level?

- This is not best practice
- The only reliable way to determine the packaging level of the _SSCC_ is to look it up in an information source

### Global Location Number (_GLN_)

#### What is a _GLN_?

- A globally unique GS1 Indentification Number used to identify [_**parties and locations**_](#parties-and-locations)

#### What is a _Corporate GLN_?

- The _GLN_ that refers to your company as a [_legal entity_](#legal-entity)

#### _GLN_ Generation

- The _GLN_ for a _legal entity_ is assigned by GS1 US when the legal entity first registers for a _GS1 Company Prefix_
- For other GLNs such as functional entities within the legal entity or physical locations, the GLN is generated by the legal entity who owns or occupies the physical location

#### Parts of a _GLN_

- A _GLN_ is a 13 digit string that is globally unique
- A _GLN_ is generated by combining the following htree components in left-to-right order
    - **GS1 Company Prefix**: A globally unique number assigned to a company/organization by GS1 US
    - **Location Reference**: A number assigned by the holder of the _GS1 Company Prefix_ to uniquely identify a legal entity, functional entity, or physical location
        - This segment is the "_serial_" part that is assigned one-by-one by the company to create a globally unique **GLN** for each entity or location
        - For a corporate **GLN** assigned by **GS1 US**, when a company first regsisters for a **GS1 Company Prefix**, **GS1 US** generates the _GLN_ usually using all zeroes for the location reference portion
        - When the company generates its own **GLN**s, it should not reuse the location reference previously used by **GS1 US** to create the corporate **GLN**
    - **Check Digit**: A single digit computed from the previous 12 digits according to the aforementioned _check-digit-calculator_

#### Uses of a GLN

- In the [_**read point**_](#read-point) of an _EPCIS_ event
- In the [_**business location**_](#business-location) of an _EPCIS_ event
    - This is typically a physical location _GLN_, though a functional entity or legal entity may be used if the reporting party does not wish to reveal more detailed location information
- In the [_**source**_](#source) and [_**destination**_](#destination) lists of an _EPCIS_ shipping or receiving event
    - Typically legal entity **GLN**s, or possible functional entity **GLN**s if the functional entity is considered to be the party to the transaction
- In the _**business transaction**_ lists of an _EPCIS_ event
    - Uniquely qualifies a business transaction identifier such as a purchase order number or invoice number
    - Typically is a legal entity **GLN**
    - Possibly a functional entity **GLN** if different functional entities within the same legal entity have independent systems for generating business transaction numbers
    - In the latter case, using different functional entity **GLN**s to qualify the business transaction numbers helps to assure the resulting identifier is globally unique
        - Even if the different functional entities happen to generate the same business transaction number
- In _**Master data**_ contained within the _EPCIS_ header of an _EPCIS_ document, to link party and location master data to the **GLN**s contained within the _EPCIS_ events
    - This avoids having to repeat information such as name and address inside of every event that referes to a party or location
- In the _**standard business document header**_ (_**SBDH**_) in the _EPCIS_ header of an _EPCIS_ document
    - To identify the party sending the message and the receiving party
    - These are typically legal entity **GLN**s, or functional entity **GLN**s if senders and receivers need to be distinguished at the functional entity level

#### How Many **GLNS** Can I Create?

|=$ GS1 Company Prefix Digits |=$ Location Reference Digits |=$ Number of GLNs (including your corporate GLN)
|$                          6 |$                          6 |$                                      1,000,000
|$                          7 |$                          5 |$                                        100,000
|$                          8 |$                          4 |$                                         10,000
|$                          9 |$                          3 |$                                          1,000
|$                         10 |$                          2 |$                                            100
|$                         11 |$                          1 |$                                             10

#### SGLN

- **SGLN** refers to an _EPC URI_ syntax for **GLN**s that is used in _EPCIS_
- The **SGLN** syntax is capable of representing a plain **GLN** (without extension) or a **GLN** plus extensions
- The "S" in **SGLN** does not stand for anything

### GS1 Company Prefix (_GCP_)

#### What is a GS1 Company Prefix?

- A unique string of 6-11 digits issued to a company by your local GS1 Member Organization
- These digits are a part of every GS1 identification number that is created
- Because the _GCP_ is different from every other company's _GCP_, the identification numbers are also globally unique

#### Does the _GCP_ uniquely identify my company or brand?

- No
- A company may use more than one _GCP_ to identify different parts of its company
- A parent company may use one _GCP_ for all of it's companies

#### Isn't there a possibility of collision due to variable length?

- No
- If GS1 assigns the 7-digit 0614141 to company A, it will not assign the 8-digit 06141411 to company B

#### What is the relationship of an FDA labeler code and _GSC_?

- The FDA National Drug Code (_NDC_) is a 10-digit identifier constructed from 3 segments
    - A 4-5 digit labeler code assigned by the FDA to a drug manufacturer
    - A product code assigned by the manufacturer
    - A package code assigned by the manufacturer
- The NDC may be embedded in a [_GTIN_](#what-is-a-gtin)

#### What is a UPC Company Prefix

- FS1 US uses the term "U.P.C. Company Prefix" to refer to the portion of a _GCP_ that is encoded in a UPC-A barcode
- The UPC Company prefix is the same as the _GCP_ with the initial "0" removed

## Glossary

##### <trade-items>Trade Items

> Products and services that may be priced, ordered, or invoiced at any point in the supply chain

##### <packaging-level>Packaging Level

> A unique quantity of a _**trade item**_.
>
> _Ex_: a bottle of 30 tablets, a case of 100 bottles of tablets, etc.

##### <homogeneous-packaging>Homogeneous Packaging

> A full bottle, case, pallet, of a singly identified _**trade item**_.
>
> _Ex_: A 30-tablet bottle of drug _XYZ_ will have one _GTN_, a 12-bottle case of _XYZ_ will have a different _GTIN_

##### <logistical-units>Logistical Units

> An item of any composition established for transprt and/or storage which needs to be tracked individually and managed through the supply chain.
>
> _Ex_: a pallet of cases picked to order, a mixed case of items picked to order, a homogeneous case of items that contains fewer than a full case, a plastic tote containing items picked to order. Unlike a _**trade item**_, each logistic unit contains different contents

##### <shipment-unit>Shipment Units

> _Ex_: Tote, pallet, etc

##### <parties-and-locations>Parties and Locations

> A legal entity (health system corporation), a functional entity (a hospital pharmacy or accounting department), or a physical location (warehouse, hospital wing, loading dock door, storage location, nursing station, etc)

##### <read-point>Read Point

> Identifies the physical location wehere a business process step (bizStep) took place

##### <business-location>Business Location

> Identifies the physical location where physical objects are expected to be following the business process described by the event

##### <source>Source

> Identifies the "sold-by" party of a transaction or the "ship-from" party of a transaction if it is a different entity from the "sold-by" party

##### <destination>Destination

> Identifies the "sold-to" party of a transaction or the "ship-to" party of a transaction if it is a different entity from the "sold-to" party

## Examples

##### <gtin-12>GTIN-12

> The 10-digit _NDC_ `0001012345` results in _GTIN-12_ `300010123455`, which can be used to identify the individual sale unit for this _NDC_

##### <gtin-14>GTIN-14

> The 10-digit _NDC_ `0001012345` results in _GTIN-14_ `10300010123452` if indicator digit "1" is used. The same _NDC_ results in _GTIN-14_ `20300010123459` if indicator digit "2" is used.

##### <serial-number>Serial Number

> A particular _GTIN_ might be assigned to identify the **trade item** "30-tablet bottle of drug XYZ". All 30-tablet bottles of drug XYZ will have the same _GTIN_, but each individual 30-tablet bottle of drug XYZ will have a different serial number, and therefore a different _SGTIN_. This way, the _SGTIN_ can be used to track and trace that one individual bottle through the supply chain.

##### <legal-entity>Legal Entity

> When you are required to identify the seller and the buyer (as parties to the transaction)
"#;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench 2", |b| b.iter(|| Parser::parse_str(MARKDOWN)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
