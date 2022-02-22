use yew::{prelude::*, function_component, html, Html};

#[function_component(Footer)]
pub fn setup_footer() -> Html {
    html! {
        <>
        <section class="flex-wrapper">
            <footer>
                <div class="top text-center">
                <a href="#nav-top">{"Back to top"}</a>
                </div>
            
                <div class="middle">
                    <div class="center">
                        <ul>
                            <li><h3>{"Get to Know Us"}</h3></li>
                            <li><a href="#">{"Careers"}</a></li>
                            <li><a href="#">{"About Amazon"}</a></li>
                            <li><a href="#">{"Investor Relations"}</a></li>
                            <li><a href="#">{"Amazon Devices"}</a></li>
                        </ul>
                        <ul>
                            <li><h3>{"Make Money with Us"}</h3></li>
                            <li><a href="#">{"Sell on Amazon"}</a></li>
                            <li><a href="#">{"Sell Your Services on Amazon"}</a></li>
                            <li><a href="#">{"Sell on Amazon Business"}</a></li>
                            <li><a href="#">{"Sell Your Apps on Amazon"}</a></li>
                            <li><a href="#">{"Become an Affiliate"}</a></li>
                            <li><a href="#">{"Advertise Your Products"}</a></li>
                            <li><a href="#">{"Self-Publish with Us"}</a></li>
                            <li><a href="#">{"Become an Amazon Vendor"}</a></li>
                            <li><a href="#">{"Sell Your Subscription on Amazon"}</a></li>
                            <li><a href="#">{" See all"}</a></li>
                        </ul>
                        <ul>
                            <li><h3>{"Amazon Payment Products"}</h3></li>
                            <li><a href="#">{"Amazon Rewards Visa Signature Cards"}</a></li>
                            <li><a href="#">{"Amazon.com Store Card"}</a></li>
                            <li><a href="#">{"Amazon.com Corporate Credit Line"}</a></li>
                            <li><a href="#">{"Shop with Points"}</a></li>
                            <li><a href="#">{"Credit Card Marketplace"}</a></li>
                            <li><a href="#">{"Reload Your Balance"}</a></li>
                            <li><a href="#">{"Amazon Currency Converter"}</a></li>
                        </ul>
                        <ul>
                            <li><h3>{"Let Us Help You"}</h3></li>
                            <li><a href="#">{"Your Account"}</a></li>
                            <li><a href="#">{"Your Orders"}</a></li>
                            <li><a href="#">{"Shipping Rates &amp; Policies"}</a></li>
                            <li><a href="#">{"Amazon Prime"}</a></li>
                            <li><a href="#">{"Returns &amp; Replacements"}</a></li>
                            <li><a href="#">{"Manage Your Content and Devices"}</a></li>
                            <li><a href="#">{"Amazon Assistant"}</a></li>
                            <li><a href="#">{"Help"}</a></li>
                        </ul>
                    </div>
                <ul class="copy text-center">
                    <li><a href="#" class="logo"></a></li>
                    <li><a href="#" class="select"><i class="fa fa-globe" aria-hidden="true"></i>{" English"}</a></li>
                    <li><a href="#" class="select"><i class="flag-icon-us"></i>{"United States"}</a></li>
                </ul>
                </div>
            
                <div class="bottom">
                    <div class="center">
                        <ul>
                            <li><a href="#">{"Amazon Music"}<br/><span>{"Stream millions"}<br/> {"of songs"}</span></a></li>
                            <li><a href="#">{"AmazonFresh"}<br/><span>{"Groceries &amp; More"}<br/>{" Right To Your Door"}</span></a></li>
                            <li><a href="#">{"Amazon Web Services"}<br/><span>{"Scalable Cloud"}<br/>{" Computing Services"}</span></a></li>
                            <li><a href="#">{"East Dane"}<br/><span>{"Designer Men 's"}<br/>{" Fashion"}</span></a></li>
                            <li><a href="#">{"Prime Now"}<br/><span>{"FREE 2-Hour Delivery"}<br/>{" on Everyday Items"}</span></a></li>
                        </ul>
                
                        <ul>
                            <li><a href="#">{"Amazon Drive"}<br/><span>{"Cloud storage"}<br/>{" from Amazon"}</span></a></li>
                            <li><a href="#">{"AmazonGlobal"}<br/><span>{"Ship Orders"}<br/>{" Internationally"}</span></a></li>
                            <li><a href="#">{"Audible"}<br/><span>{"Download"}<br/>{" Audio Books"}</span></a></li>
                            <li><a href="#">{"Fabric"}<br/><span>{"Sewing, Quilting"}<br/>{" &amp; Knitting"}</span></a></li>
                            <li><a href="#">{"Prime Photos"}<br/><span>{"Unlimited Photo Storage"}<br/>{" Free With Prime"}</span></a></li>
                            <li><a href="#">{"Woot!"}<br/><span>{"Deals and "}<br/>{" Shenanigans"}</span></a></li>
                        </ul>
                
                        <ul>
                            <li><a href="#">{"6pm"}<br/><span>{"Score deals"}<br/>{" on fashion brands"}</span></a></li>
                            <li><a href="#">{"Home Services"}<br/><span>{"Handpicked Pros"}<br/>{" Happiness Guarantee"}</span></a></li>
                            <li><a href="#">{"Book Depository"}<br/><span>{"Books With Free"}<br/>{" Delivery Worldwide"}</span></a></li>
                            <li><a href="#">{"Goodreads"}<br/><span>{"Book reviews"}<br/>{" &amp; recommendations"}</span></a></li>
                            <li><a href="#">{"Shopbop"}<br/><span>{"Designer"}<br/>{" Fashion Brands"}</span></a></li>
                            <li><a href="#">{"Zappos"}<br/><span>{"Shoes &amp;"}<br/>{" Clothing"}</span></a></li>
                        </ul>
                
                        <ul>
                            <li><a href="#">{"AbeBooks"}<br/><span>{"Books, art"}<br/>{" &amp; collectibles"}</span></a></li>
                            <li><a href="#">{"Amazon Inspire"}<br/><span>{"Free Digital Educational"}<br/>{"  Resources"}</span></a></li>
                            <li><a href="#">{"Box Office Mojo"}<br/><span>{"Find Movie"}<br/>{" Box Office Data"}</span></a></li>
                            <li><a href="#">{"IMDb"}<br/><span>{"Movies, TV"}<br/>{" &amp; Celebrities"}</span></a></li>
                            <li><a href="#">{"TenMarks.com"}<br/><span>{"Math Activities"}<br/>{" for Kids &amp; Schools"}</span></a></li>
                            <li><a href="#">{"Souq.com"}<br/><span>{"Shop Online in"}<br/>{" the Middle East"}</span></a></li>
                        </ul>
                
                        <ul>
                            <li><a href="#">{"ACX "}<br/><span>{"Audiobook Publishing"}<br/>{" Made Easy"}</span></a></li>
                            <li><a href="#">{"Amazon Rapids"}<br/><span>{"Fun stories for"}<br/>{"  kids on the go"}</span></a></li>
                            <li><a href="#">{"ComiXology"}<br/><span>{"Thousands of"}<br/>{" Digital Comics"}</span></a></li>
                            <li><a href="#">{"IMDbPro"}<br/><span>{"Get Info Entertainment"}<br/>{" Professionals Need"}</span></a></li>
                            <li><a href="#">{"Warehouse Deals"}<br/><span>{"Open-Box"}<br/>{" Discounts"}</span></a></li>
                            <li><a href="#">{"Subscribe with Amazon"}<br/><span>{"Discover &amp; try"}<br/>{" subscription services"}</span></a></li>
                        </ul>
                
                        <ul>
                            <li><a href="#">{"Alexa"}<br/><span>{"Actionable Analytics"}<br/>{" for the Web"}</span></a></li>
                            <li><a href="#">{"Amazon Restaurants"}<br/><span>{"Food delivery from"}<br/>{" local restaurants"}</span></a></li>
                            <li><a href="#">{"CreateSpace"}<br/><span>{"Indie Print Publishing"}<br/>{" Made Easy"}</span></a></li>
                            <li><a href="#">{"Junglee.com"}<br/><span>{"Shop Online"}<br/>{" in India"}</span></a></li>
                            <li><a href="#">{"Whispercast"}<br/><span>{"Discover &amp; Distribute"}<br/>{" Digital Content"}</span></a></li>
                        </ul>
                
                        <ul>
                            <li><a href="#">{"Amazon Business"}<br/><span>{"Everything For"}<br/>{" Your Business"}</span></a></li>
                            <li><a href="#">{"Amazon Video Direct"}<br/><span>{"Video Distribution"}<br/>{" Made Easy"}</span></a></li>
                            <li><a href="#">{"DPReview"}<br/><span>{"Digital"}<br/>{" Photography"}</span></a></li>
                            <li><a href="#">{"Kindle Direct Publishing"}<br/><span>{"Indie Digital Publishing"}<br/>{" Made Easy"}</span></a></li>
                            <li><a href="#">{"Withoutabox"}<br/><span>{"Submit to"}<br/>{" Film Festivals"}</span></a></li>
                        </ul>
                
                        <ul class="copy text-center">
                            <li><a href="#">{"Conditions of Use"}</a></li>
                            <li><a href="#">{"Privacy Notice"}</a></li>
                            <li><a href="#">{"Interest-Based Ads"}</a></li>
                            <li>{"&copy; 1996-2017, Amazon.com, Inc. or its affiliates"}</li>
                        </ul>
                    </div>
                </div>
            </footer>
            </section>
        </>
    }
}