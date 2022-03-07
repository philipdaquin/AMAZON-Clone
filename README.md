# Amazon-Clone [WIP]
Left To Do: 
-  FIX Fetch Request on (Frontend)
-  TEST Stripe Payment Service + User Order History 
-  Deploy to AWS and Use Terraform 
-  Elastic Search
-  Refactor REST APIs to GraphQL, ensuring to document this stage
## High-Level Design: Main Components
- Inventory
- Delivery/Notification Service
- Authentication service 
- Payment Services 
- Cart 
### Requirements
1. Users should be able to add new products to sell 
2. Users should be able to search for products by their name or category 
3. Users can search and view all the products, but they will have to become a registered member to buy a product.
4. Users should be able to add/ remove/ modify product items in their shopping cart 
5. Users can rate and add a review for a product.
6. Users should be notifications whenever there is a change in the order or shipping status
7. Users should be able to track their shipment to see the current state of their order 
## Tech-stack
- Yew         
- Actix           
- Graphql        
- Diesel           
- PostgreSQL
- Terraform 
- AWS


