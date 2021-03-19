use diesel::sql_types::*;
use diesel_full_text_search::{TsVector as Tsvector};
use bigdecimal::BigDecimal;

table! {
    accommodation_class (accommodation_class_id) {
        accommodation_class_id -> Varchar,
        parent_class_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    accommodation_map (accommodation_map_id) {
        accommodation_map_id -> Varchar,
        accommodation_class_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        accommodation_map_type_id -> Nullable<Varchar>,
        number_of_spaces -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    accommodation_map_type (accommodation_map_type_id) {
        accommodation_map_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    accommodation_spot (accommodation_spot_id) {
        accommodation_spot_id -> Varchar,
        accommodation_class_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        number_of_spaces -> Nullable<Numeric>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    acctg_trans (acctg_trans_id) {
        acctg_trans_id -> Varchar,
        acctg_trans_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        transaction_date -> Nullable<Timestamptz>,
        is_posted -> Nullable<Bpchar>,
        posted_date -> Nullable<Timestamptz>,
        scheduled_posting_date -> Nullable<Timestamptz>,
        gl_journal_id -> Nullable<Varchar>,
        gl_fiscal_type_id -> Nullable<Varchar>,
        voucher_ref -> Nullable<Varchar>,
        voucher_date -> Nullable<Timestamptz>,
        group_status_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        inventory_item_id -> Nullable<Varchar>,
        physical_inventory_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        invoice_id -> Nullable<Varchar>,
        payment_id -> Nullable<Varchar>,
        fin_account_trans_id -> Nullable<Varchar>,
        shipment_id -> Nullable<Varchar>,
        receipt_id -> Nullable<Varchar>,
        work_effort_id -> Nullable<Varchar>,
        their_acctg_trans_id -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    acctg_trans_attribute (acctg_trans_id, attr_name) {
        acctg_trans_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    acctg_trans_entry (acctg_trans_id, acctg_trans_entry_seq_id) {
        acctg_trans_id -> Varchar,
        acctg_trans_entry_seq_id -> Varchar,
        acctg_trans_entry_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        voucher_ref -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        their_party_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        their_product_id -> Nullable<Varchar>,
        inventory_item_id -> Nullable<Varchar>,
        gl_account_type_id -> Nullable<Varchar>,
        gl_account_id -> Nullable<Varchar>,
        organization_party_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        orig_amount -> Nullable<Numeric>,
        orig_currency_uom_id -> Nullable<Varchar>,
        debit_credit_flag -> Nullable<Bpchar>,
        due_date -> Nullable<Date>,
        group_id -> Nullable<Varchar>,
        tax_id -> Nullable<Varchar>,
        reconcile_status_id -> Nullable<Varchar>,
        settlement_term_id -> Nullable<Varchar>,
        is_summary -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    acctg_trans_entry_type (acctg_trans_entry_type_id) {
        acctg_trans_entry_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    acctg_trans_type (acctg_trans_type_id) {
        acctg_trans_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    acctg_trans_type_attr (acctg_trans_type_id, attr_name) {
        acctg_trans_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    addendum (addendum_id) {
        addendum_id -> Varchar,
        agreement_id -> Nullable<Varchar>,
        agreement_item_seq_id -> Nullable<Varchar>,
        addendum_creation_date -> Nullable<Timestamptz>,
        addendum_effective_date -> Nullable<Timestamptz>,
        addendum_text -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    address_match_map (map_key, map_value) {
        map_key -> Varchar,
        map_value -> Varchar,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    affiliate (party_id) {
        party_id -> Varchar,
        affiliate_name -> Nullable<Varchar>,
        affiliate_description -> Nullable<Varchar>,
        year_established -> Nullable<Varchar>,
        site_type -> Nullable<Varchar>,
        site_page_views -> Nullable<Varchar>,
        site_visitors -> Nullable<Varchar>,
        date_time_created -> Nullable<Timestamptz>,
        date_time_approved -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement (agreement_id) {
        agreement_id -> Varchar,
        product_id -> Nullable<Varchar>,
        party_id_from -> Nullable<Varchar>,
        party_id_to -> Nullable<Varchar>,
        role_type_id_from -> Nullable<Varchar>,
        role_type_id_to -> Nullable<Varchar>,
        agreement_type_id -> Nullable<Varchar>,
        agreement_date -> Nullable<Timestamptz>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        text_data -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_attribute (agreement_id, attr_name) {
        agreement_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_content (content_id, agreement_id, agreement_item_seq_id, agreement_content_type_id, from_date) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        agreement_content_type_id -> Varchar,
        content_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_content_type (agreement_content_type_id) {
        agreement_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_employment_appl (agreement_id, agreement_item_seq_id, party_id_to, party_id_from, role_type_id_to, role_type_id_from, from_date) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        party_id_from -> Varchar,
        party_id_to -> Varchar,
        role_type_id_from -> Varchar,
        role_type_id_to -> Varchar,
        from_date -> Timestamptz,
        agreement_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_facility_appl (agreement_id, agreement_item_seq_id, facility_id) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        facility_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_geographical_applic (agreement_id, agreement_item_seq_id, geo_id) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        geo_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_item (agreement_id, agreement_item_seq_id) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        agreement_item_type_id -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        agreement_text -> Nullable<Text>,
        agreement_image -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_item_attribute (agreement_id, agreement_item_seq_id, attr_name) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_item_type (agreement_item_type_id) {
        agreement_item_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_item_type_attr (agreement_item_type_id, attr_name) {
        agreement_item_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_party_applic (agreement_id, agreement_item_seq_id, party_id) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        party_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_product_appl (agreement_id, agreement_item_seq_id, product_id) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        product_id -> Varchar,
        price -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_promo_appl (agreement_id, agreement_item_seq_id, product_promo_id, from_date) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        product_promo_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_role (agreement_id, party_id, role_type_id) {
        agreement_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_term (agreement_term_id) {
        agreement_term_id -> Varchar,
        term_type_id -> Nullable<Varchar>,
        agreement_id -> Nullable<Varchar>,
        agreement_item_seq_id -> Nullable<Varchar>,
        invoice_item_type_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        term_value -> Nullable<Numeric>,
        term_days -> Nullable<Numeric>,
        text_value -> Nullable<Varchar>,
        min_quantity -> Nullable<Float8>,
        max_quantity -> Nullable<Float8>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_term_attribute (agreement_term_id, attr_name) {
        agreement_term_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_type (agreement_type_id) {
        agreement_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_type_attr (agreement_type_id, attr_name) {
        agreement_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    agreement_work_effort_applic (agreement_id, agreement_item_seq_id, work_effort_id) {
        agreement_id -> Varchar,
        agreement_item_seq_id -> Varchar,
        work_effort_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    allocation_plan_header (plan_id, product_id) {
        plan_id -> Varchar,
        product_id -> Varchar,
        plan_type_id -> Nullable<Varchar>,
        plan_name -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    allocation_plan_item (plan_id, plan_item_seq_id, product_id) {
        plan_id -> Varchar,
        plan_item_seq_id -> Varchar,
        status_id -> Nullable<Varchar>,
        plan_method_enum_id -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        product_id -> Varchar,
        allocated_quantity -> Nullable<Numeric>,
        priority_seq_id -> Nullable<Varchar>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    allocation_plan_type (plan_type_id) {
        plan_type_id -> Varchar,
        description -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    application_sandbox (application_id) {
        application_id -> Varchar,
        work_effort_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        runtime_data_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    audio_data_resource (data_resource_id) {
        data_resource_id -> Varchar,
        audio_data -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    benefit_type (benefit_type_id) {
        benefit_type_id -> Varchar,
        benefit_name -> Nullable<Varchar>,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        employer_paid_percentage -> Nullable<Float8>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    billing_account (billing_account_id) {
        billing_account_id -> Varchar,
        account_limit -> Nullable<Numeric>,
        account_currency_uom_id -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        external_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    billing_account_role (billing_account_id, party_id, role_type_id, from_date) {
        billing_account_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    billing_account_term (billing_account_term_id) {
        billing_account_term_id -> Varchar,
        billing_account_id -> Nullable<Varchar>,
        term_type_id -> Nullable<Varchar>,
        term_value -> Nullable<Numeric>,
        term_days -> Nullable<Numeric>,
        uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    billing_account_term_attr (billing_account_term_id, attr_name) {
        billing_account_term_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    browser_type (browser_type_id) {
        browser_type_id -> Varchar,
        browser_name -> Nullable<Varchar>,
        browser_version -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget (budget_id) {
        budget_id -> Varchar,
        budget_type_id -> Nullable<Varchar>,
        custom_time_period_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_attribute (budget_id, attr_name) {
        budget_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_item (budget_id, budget_item_seq_id) {
        budget_id -> Varchar,
        budget_item_seq_id -> Varchar,
        budget_item_type_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        purpose -> Nullable<Varchar>,
        justification -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_item_attribute (budget_id, budget_item_seq_id, attr_name) {
        budget_id -> Varchar,
        budget_item_seq_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_item_type (budget_item_type_id) {
        budget_item_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_item_type_attr (budget_item_type_id, attr_name) {
        budget_item_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_review (budget_id, budget_review_id, party_id, budget_review_result_type_id) {
        budget_id -> Varchar,
        budget_review_id -> Varchar,
        party_id -> Varchar,
        budget_review_result_type_id -> Varchar,
        review_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_review_result_type (budget_review_result_type_id) {
        budget_review_result_type_id -> Varchar,
        description -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_revision (budget_id, revision_seq_id) {
        budget_id -> Varchar,
        revision_seq_id -> Varchar,
        date_revised -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_revision_impact (budget_id, budget_item_seq_id, revision_seq_id) {
        budget_id -> Varchar,
        budget_item_seq_id -> Varchar,
        revision_seq_id -> Varchar,
        revised_amount -> Nullable<Numeric>,
        add_delete_flag -> Nullable<Bpchar>,
        revision_reason -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_role (budget_id, party_id, role_type_id) {
        budget_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_scenario (budget_scenario_id) {
        budget_scenario_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_scenario_application (budget_scenario_applic_id, budget_scenario_id) {
        budget_scenario_applic_id -> Varchar,
        budget_scenario_id -> Varchar,
        budget_id -> Nullable<Varchar>,
        budget_item_seq_id -> Nullable<Varchar>,
        amount_change -> Nullable<Numeric>,
        percentage_change -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_scenario_rule (budget_scenario_id, budget_item_type_id) {
        budget_scenario_id -> Varchar,
        budget_item_type_id -> Varchar,
        amount_change -> Nullable<Numeric>,
        percentage_change -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_status (budget_id, status_id) {
        budget_id -> Varchar,
        status_id -> Varchar,
        status_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_type (budget_type_id) {
        budget_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    budget_type_attr (budget_type_id, attr_name) {
        budget_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    carrier_shipment_box_type (shipment_box_type_id, party_id) {
        shipment_box_type_id -> Varchar,
        party_id -> Varchar,
        packaging_type_code -> Nullable<Varchar>,
        oversize_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    carrier_shipment_method (shipment_method_type_id, party_id, role_type_id) {
        shipment_method_type_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        sequence_number -> Nullable<Numeric>,
        carrier_service_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cart_abandoned_line (visit_id, cart_abandoned_line_seq_id) {
        visit_id -> Varchar,
        cart_abandoned_line_seq_id -> Varchar,
        product_id -> Nullable<Varchar>,
        prod_catalog_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        reserv_start -> Nullable<Timestamptz>,
        reserv_length -> Nullable<Numeric>,
        reserv_persons -> Nullable<Numeric>,
        unit_price -> Nullable<Numeric>,
        reserv2nd_p_p_perc -> Nullable<Numeric>,
        reserv_nth_p_p_perc -> Nullable<Numeric>,
        config_id -> Nullable<Varchar>,
        total_with_adjustments -> Nullable<Numeric>,
        was_reserved -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    catalina_session (session_id) {
        session_id -> Varchar,
        session_size -> Nullable<Numeric>,
        session_info -> Nullable<Bytea>,
        is_valid -> Nullable<Bpchar>,
        max_idle -> Nullable<Numeric>,
        last_accessed -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    character_set (character_set_id) {
        character_set_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    check_account (payment_method_id) {
        payment_method_id -> Varchar,
        bank_name -> Nullable<Varchar>,
        routing_number -> Nullable<Varchar>,
        account_type -> Nullable<Varchar>,
        account_number -> Nullable<Varchar>,
        name_on_account -> Nullable<Varchar>,
        company_name_on_account -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        branch_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    comm_content_assoc_type (comm_content_assoc_type_id) {
        comm_content_assoc_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    comm_event_content_assoc (content_id, communication_event_id, from_date) {
        content_id -> Varchar,
        communication_event_id -> Varchar,
        comm_content_assoc_type_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event (communication_event_id) {
        communication_event_id -> Varchar,
        communication_event_type_id -> Nullable<Varchar>,
        orig_comm_event_id -> Nullable<Varchar>,
        parent_comm_event_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        contact_mech_type_id -> Nullable<Varchar>,
        contact_mech_id_from -> Nullable<Varchar>,
        contact_mech_id_to -> Nullable<Varchar>,
        role_type_id_from -> Nullable<Varchar>,
        role_type_id_to -> Nullable<Varchar>,
        party_id_from -> Nullable<Varchar>,
        party_id_to -> Nullable<Varchar>,
        entry_date -> Nullable<Timestamptz>,
        datetime_started -> Nullable<Timestamptz>,
        datetime_ended -> Nullable<Timestamptz>,
        subject -> Nullable<Varchar>,
        content_mime_type_id -> Nullable<Varchar>,
        content -> Nullable<Text>,
        note -> Nullable<Varchar>,
        reason_enum_id -> Nullable<Varchar>,
        contact_list_id -> Nullable<Varchar>,
        header_string -> Nullable<Text>,
        from_string -> Nullable<Text>,
        to_string -> Nullable<Text>,
        cc_string -> Nullable<Text>,
        bcc_string -> Nullable<Text>,
        message_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_order (order_id, communication_event_id) {
        order_id -> Varchar,
        communication_event_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_product (product_id, communication_event_id) {
        product_id -> Varchar,
        communication_event_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_prp_typ (communication_event_prp_typ_id) {
        communication_event_prp_typ_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_purpose (communication_event_prp_typ_id, communication_event_id) {
        communication_event_prp_typ_id -> Varchar,
        communication_event_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_return (return_id, communication_event_id) {
        return_id -> Varchar,
        communication_event_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_role (communication_event_id, party_id, role_type_id) {
        communication_event_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        contact_mech_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_type (communication_event_type_id) {
        communication_event_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        contact_mech_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    communication_event_work_eff (work_effort_id, communication_event_id) {
        work_effort_id -> Varchar,
        communication_event_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    config_option_product_option (config_id, config_item_id, config_option_id, sequence_num, product_id) {
        config_id -> Varchar,
        config_item_id -> Varchar,
        sequence_num -> Numeric,
        config_option_id -> Varchar,
        product_id -> Varchar,
        product_option_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_list (contact_list_id) {
        contact_list_id -> Varchar,
        contact_list_type_id -> Nullable<Varchar>,
        contact_mech_type_id -> Nullable<Varchar>,
        marketing_campaign_id -> Nullable<Varchar>,
        contact_list_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        is_public -> Nullable<Bpchar>,
        single_use -> Nullable<Bpchar>,
        owner_party_id -> Nullable<Varchar>,
        verify_email_from -> Nullable<Varchar>,
        verify_email_screen -> Nullable<Varchar>,
        verify_email_subject -> Nullable<Varchar>,
        verify_email_web_site_id -> Nullable<Varchar>,
        opt_out_screen -> Nullable<Varchar>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_list_comm_status (contact_list_id, communication_event_id, contact_mech_id) {
        contact_list_id -> Varchar,
        communication_event_id -> Varchar,
        contact_mech_id -> Varchar,
        party_id -> Nullable<Varchar>,
        message_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_list_party (contact_list_id, party_id, from_date) {
        contact_list_id -> Varchar,
        party_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Varchar>,
        preferred_contact_mech_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_list_party_status (contact_list_id, party_id, from_date, status_date) {
        contact_list_id -> Varchar,
        party_id -> Varchar,
        from_date -> Timestamptz,
        status_date -> Timestamptz,
        status_id -> Nullable<Varchar>,
        set_by_user_login_id -> Nullable<Varchar>,
        opt_in_verify_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_list_type (contact_list_type_id) {
        contact_list_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_mech (contact_mech_id) {
        contact_mech_id -> Varchar,
        contact_mech_type_id -> Nullable<Varchar>,
        info_string -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_mech_attribute (contact_mech_id, attr_name) {
        contact_mech_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_mech_link (contact_mech_id_from, contact_mech_id_to) {
        contact_mech_id_from -> Varchar,
        contact_mech_id_to -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_mech_purpose_type (contact_mech_purpose_type_id) {
        contact_mech_purpose_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_mech_type (contact_mech_type_id) {
        contact_mech_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_mech_type_attr (contact_mech_type_id, attr_name) {
        contact_mech_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    contact_mech_type_purpose (contact_mech_type_id, contact_mech_purpose_type_id) {
        contact_mech_type_id -> Varchar,
        contact_mech_purpose_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    container (container_id) {
        container_id -> Varchar,
        container_type_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    container_geo_point (container_id, geo_point_id, from_date) {
        container_id -> Varchar,
        geo_point_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    container_type (container_type_id) {
        container_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content (content_id) {
        content_id -> Varchar,
        content_type_id -> Nullable<Varchar>,
        owner_content_id -> Nullable<Varchar>,
        decorator_content_id -> Nullable<Varchar>,
        instance_of_content_id -> Nullable<Varchar>,
        data_resource_id -> Nullable<Varchar>,
        template_data_resource_id -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        privilege_enum_id -> Nullable<Varchar>,
        service_name -> Nullable<Varchar>,
        custom_method_id -> Nullable<Varchar>,
        content_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        locale_string -> Nullable<Varchar>,
        mime_type_id -> Nullable<Varchar>,
        character_set_id -> Nullable<Varchar>,
        child_leaf_count -> Nullable<Numeric>,
        child_branch_count -> Nullable<Numeric>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_approval (content_approval_id) {
        content_approval_id -> Varchar,
        content_id -> Nullable<Varchar>,
        content_revision_seq_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        approval_status_id -> Nullable<Varchar>,
        approval_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_assoc (content_id, content_id_to, content_assoc_type_id, from_date) {
        content_id -> Varchar,
        content_id_to -> Varchar,
        content_assoc_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        content_assoc_predicate_id -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        map_key -> Nullable<Varchar>,
        upper_coordinate -> Nullable<Numeric>,
        left_coordinate -> Nullable<Numeric>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_assoc_predicate (content_assoc_predicate_id) {
        content_assoc_predicate_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_assoc_type (content_assoc_type_id) {
        content_assoc_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_attribute (content_id, attr_name) {
        content_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_keyword (content_id, keyword) {
        content_id -> Varchar,
        keyword -> Varchar,
        relevancy_weight -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_meta_data (content_id, meta_data_predicate_id) {
        content_id -> Varchar,
        meta_data_predicate_id -> Varchar,
        meta_data_value -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_operation (content_operation_id) {
        content_operation_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_purpose (content_id, content_purpose_type_id) {
        content_id -> Varchar,
        content_purpose_type_id -> Varchar,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_purpose_operation (content_purpose_type_id, content_operation_id, role_type_id, status_id, privilege_enum_id) {
        content_purpose_type_id -> Varchar,
        content_operation_id -> Varchar,
        role_type_id -> Varchar,
        status_id -> Varchar,
        privilege_enum_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_purpose_type (content_purpose_type_id) {
        content_purpose_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_revision (content_id, content_revision_seq_id) {
        content_id -> Varchar,
        content_revision_seq_id -> Varchar,
        committed_by_party_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_revision_item (content_id, content_revision_seq_id, item_content_id) {
        content_id -> Varchar,
        content_revision_seq_id -> Varchar,
        item_content_id -> Varchar,
        old_data_resource_id -> Nullable<Varchar>,
        new_data_resource_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_role (content_id, party_id, role_type_id, from_date) {
        content_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_search_constraint (content_search_result_id, constraint_seq_id) {
        content_search_result_id -> Varchar,
        constraint_seq_id -> Varchar,
        constraint_name -> Nullable<Varchar>,
        info_string -> Nullable<Varchar>,
        include_sub_categories -> Nullable<Bpchar>,
        is_and -> Nullable<Bpchar>,
        any_prefix -> Nullable<Bpchar>,
        any_suffix -> Nullable<Bpchar>,
        remove_stems -> Nullable<Bpchar>,
        low_value -> Nullable<Varchar>,
        high_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_search_result (content_search_result_id) {
        content_search_result_id -> Varchar,
        visit_id -> Nullable<Varchar>,
        order_by_name -> Nullable<Varchar>,
        is_ascending -> Nullable<Bpchar>,
        num_results -> Nullable<Numeric>,
        seconds_total -> Nullable<Float8>,
        search_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_type (content_type_id) {
        content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    content_type_attr (content_type_id, attr_name) {
        content_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cost_component (cost_component_id) {
        cost_component_id -> Varchar,
        cost_component_type_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        product_feature_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        geo_id -> Nullable<Varchar>,
        work_effort_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        cost_component_calc_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        cost -> Nullable<Numeric>,
        cost_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cost_component_attribute (cost_component_id, attr_name) {
        cost_component_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cost_component_calc (cost_component_calc_id) {
        cost_component_calc_id -> Varchar,
        description -> Nullable<Varchar>,
        cost_gl_account_type_id -> Nullable<Varchar>,
        offsetting_gl_account_type_id -> Nullable<Varchar>,
        fixed_cost -> Nullable<Numeric>,
        variable_cost -> Nullable<Numeric>,
        per_milli_second -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        cost_custom_method_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cost_component_type (cost_component_type_id) {
        cost_component_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cost_component_type_attr (cost_component_type_id, attr_name) {
        cost_component_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    country_address_format (geo_id) {
        geo_id -> Varchar,
        geo_assoc_type_id -> Nullable<Varchar>,
        require_state_province_id -> Nullable<Varchar>,
        require_postal_code -> Nullable<Bpchar>,
        postal_code_regex -> Nullable<Varchar>,
        has_postal_code_ext -> Nullable<Bpchar>,
        require_postal_code_ext -> Nullable<Bpchar>,
        address_format -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    country_capital (country_code) {
        country_code -> Varchar,
        #[sql_name = "country_capital_name"]
        country_capital_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    country_code (country_code_id) {
        #[sql_name = "country_code"]
        country_code_id -> Varchar,
        country_abbr -> Nullable<Varchar>,
        country_number -> Nullable<Varchar>,
        country_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    country_tele_code (country_code) {
        country_code -> Varchar,
        tele_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    credit_card (payment_method_id) {
        payment_method_id -> Varchar,
        card_type -> Nullable<Varchar>,
        card_number -> Nullable<Varchar>,
        valid_from_date -> Nullable<Varchar>,
        expire_date -> Nullable<Varchar>,
        issue_number -> Nullable<Varchar>,
        company_name_on_card -> Nullable<Varchar>,
        title_on_card -> Nullable<Varchar>,
        first_name_on_card -> Nullable<Varchar>,
        middle_name_on_card -> Nullable<Varchar>,
        last_name_on_card -> Nullable<Varchar>,
        suffix_on_card -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        consecutive_failed_auths -> Nullable<Numeric>,
        last_failed_auth_date -> Nullable<Timestamptz>,
        consecutive_failed_nsf -> Nullable<Numeric>,
        last_failed_nsf_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    credit_card_type_gl_account (card_type, organization_party_id) {
        card_type -> Varchar,
        organization_party_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request (cust_request_id) {
        cust_request_id -> Varchar,
        cust_request_type_id -> Nullable<Varchar>,
        cust_request_category_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        from_party_id -> Nullable<Varchar>,
        priority -> Nullable<Numeric>,
        cust_request_date -> Nullable<Timestamptz>,
        response_required_date -> Nullable<Timestamptz>,
        cust_request_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        maximum_amount_uom_id -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        sales_channel_enum_id -> Nullable<Varchar>,
        fulfill_contact_mech_id -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        open_date_time -> Nullable<Timestamptz>,
        closed_date_time -> Nullable<Timestamptz>,
        internal_comment -> Nullable<Varchar>,
        reason -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        cust_estimated_milli_seconds -> Nullable<Float8>,
        cust_sequence_num -> Nullable<Numeric>,
        parent_cust_request_id -> Nullable<Varchar>,
        billed -> Nullable<Bpchar>,
    }
}

table! {
    cust_request_attribute (cust_request_id, attr_name) {
        cust_request_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_category (cust_request_category_id) {
        cust_request_category_id -> Varchar,
        cust_request_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_comm_event (cust_request_id, communication_event_id) {
        cust_request_id -> Varchar,
        communication_event_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_content (cust_request_id, content_id, from_date) {
        cust_request_id -> Varchar,
        content_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_item (cust_request_id, cust_request_item_seq_id) {
        cust_request_id -> Varchar,
        cust_request_item_seq_id -> Varchar,
        cust_request_resolution_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        priority -> Nullable<Numeric>,
        sequence_num -> Nullable<Numeric>,
        required_by_date -> Nullable<Timestamptz>,
        product_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        selected_amount -> Nullable<Numeric>,
        maximum_amount -> Nullable<Numeric>,
        reserv_start -> Nullable<Timestamptz>,
        reserv_length -> Nullable<Numeric>,
        reserv_persons -> Nullable<Numeric>,
        config_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        story -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_item_note (cust_request_id, cust_request_item_seq_id, note_id) {
        cust_request_id -> Varchar,
        cust_request_item_seq_id -> Varchar,
        note_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_item_work_effort (cust_request_id, cust_request_item_seq_id, work_effort_id) {
        cust_request_id -> Varchar,
        cust_request_item_seq_id -> Varchar,
        work_effort_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_note (cust_request_id, note_id) {
        cust_request_id -> Varchar,
        note_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_party (cust_request_id, party_id, role_type_id, from_date) {
        cust_request_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_resolution (cust_request_resolution_id) {
        cust_request_resolution_id -> Varchar,
        cust_request_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_status (cust_request_status_id) {
        cust_request_status_id -> Varchar,
        status_id -> Nullable<Varchar>,
        cust_request_id -> Nullable<Varchar>,
        cust_request_item_seq_id -> Nullable<Varchar>,
        status_date -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_type (cust_request_type_id) {
        cust_request_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_type_attr (cust_request_type_id, attr_name) {
        cust_request_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    cust_request_work_effort (cust_request_id, work_effort_id) {
        cust_request_id -> Varchar,
        work_effort_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    custom_method (custom_method_id) {
        custom_method_id -> Varchar,
        custom_method_type_id -> Nullable<Varchar>,
        custom_method_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    custom_method_type (custom_method_type_id) {
        custom_method_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    custom_screen (custom_screen_id) {
        custom_screen_id -> Varchar,
        custom_screen_type_id -> Nullable<Varchar>,
        custom_screen_name -> Nullable<Varchar>,
        custom_screen_location -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    custom_screen_type (custom_screen_type_id) {
        custom_screen_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    custom_time_period (custom_time_period_id) {
        custom_time_period_id -> Varchar,
        parent_period_id -> Nullable<Varchar>,
        period_type_id -> Nullable<Varchar>,
        period_num -> Nullable<Numeric>,
        period_name -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        is_closed -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        organization_party_id -> Nullable<Varchar>,
    }
}

table! {
    data_category (data_category_id) {
        data_category_id -> Varchar,
        parent_category_id -> Nullable<Varchar>,
        category_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_resource (data_resource_id) {
        data_resource_id -> Varchar,
        data_resource_type_id -> Nullable<Varchar>,
        data_template_type_id -> Nullable<Varchar>,
        data_category_id -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        data_resource_name -> Nullable<Varchar>,
        locale_string -> Nullable<Varchar>,
        mime_type_id -> Nullable<Varchar>,
        character_set_id -> Nullable<Varchar>,
        object_info -> Nullable<Varchar>,
        survey_id -> Nullable<Varchar>,
        survey_response_id -> Nullable<Varchar>,
        related_detail_id -> Nullable<Varchar>,
        is_public -> Nullable<Bpchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_resource_attribute (data_resource_id, attr_name) {
        data_resource_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_resource_meta_data (data_resource_id, meta_data_predicate_id) {
        data_resource_id -> Varchar,
        meta_data_predicate_id -> Varchar,
        meta_data_value -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_resource_purpose (data_resource_id, content_purpose_type_id) {
        data_resource_id -> Varchar,
        content_purpose_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_resource_role (data_resource_id, party_id, role_type_id, from_date) {
        data_resource_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_resource_type (data_resource_type_id) {
        data_resource_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_resource_type_attr (data_resource_type_id, attr_name) {
        data_resource_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_source (data_source_id) {
        data_source_id -> Varchar,
        data_source_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_source_type (data_source_type_id) {
        data_source_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    data_template_type (data_template_type_id) {
        data_template_type_id -> Varchar,
        description -> Nullable<Varchar>,
        extension -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    deduction (deduction_id) {
        deduction_id -> Varchar,
        deduction_type_id -> Nullable<Varchar>,
        payment_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    deduction_type (deduction_type_id) {
        deduction_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    deliverable (deliverable_id) {
        deliverable_id -> Varchar,
        deliverable_type_id -> Nullable<Varchar>,
        deliverable_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    deliverable_type (deliverable_type_id) {
        deliverable_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    delivery (delivery_id) {
        delivery_id -> Varchar,
        origin_facility_id -> Nullable<Varchar>,
        dest_facility_id -> Nullable<Varchar>,
        actual_start_date -> Nullable<Timestamptz>,
        actual_arrival_date -> Nullable<Timestamptz>,
        estimated_start_date -> Nullable<Timestamptz>,
        estimated_arrival_date -> Nullable<Timestamptz>,
        fixed_asset_id -> Nullable<Varchar>,
        start_mileage -> Nullable<Numeric>,
        end_mileage -> Nullable<Numeric>,
        fuel_used -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    desired_feature (desired_feature_id, requirement_id) {
        desired_feature_id -> Varchar,
        requirement_id -> Varchar,
        product_feature_id -> Nullable<Varchar>,
        optional_ind -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    document (document_id) {
        document_id -> Varchar,
        document_type_id -> Nullable<Varchar>,
        date_created -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        document_location -> Nullable<Varchar>,
        document_text -> Nullable<Varchar>,
        image_data -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    document_attribute (document_id, attr_name) {
        document_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    document_type (document_type_id) {
        document_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    document_type_attr (document_type_id, attr_name) {
        document_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    ebay_config (product_store_id) {
        product_store_id -> Varchar,
        dev_id -> Nullable<Varchar>,
        app_id -> Nullable<Varchar>,
        cert_id -> Nullable<Varchar>,
        token -> Nullable<Text>,
        compatibility_level -> Nullable<Varchar>,
        site_id -> Nullable<Varchar>,
        xml_gateway_uri -> Nullable<Varchar>,
        custom_xml -> Nullable<Text>,
        web_site_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    ebay_shipping_method (shipment_method_name, product_store_id) {
        shipment_method_name -> Varchar,
        product_store_id -> Varchar,
        amount -> Nullable<Numeric>,
        additional_amount -> Nullable<Numeric>,
        additional_percent -> Nullable<Numeric>,
        method_type_enum_id -> Nullable<Varchar>,
        carrier_party_id -> Nullable<Varchar>,
        shipment_method_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    eft_account (payment_method_id) {
        payment_method_id -> Varchar,
        bank_name -> Nullable<Varchar>,
        routing_number -> Nullable<Varchar>,
        account_type -> Nullable<Varchar>,
        account_number -> Nullable<Varchar>,
        name_on_account -> Nullable<Varchar>,
        company_name_on_account -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        years_at_bank -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    electronic_text (data_resource_id) {
        data_resource_id -> Varchar,
        text_data -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    email_address_verification (email_address) {
        email_address -> Varchar,
        verify_hash -> Nullable<Varchar>,
        expire_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    email_template_setting (email_template_setting_id) {
        email_template_setting_id -> Varchar,
        email_type -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        body_screen_location -> Nullable<Varchar>,
        xslfo_attach_screen_location -> Nullable<Varchar>,
        from_address -> Nullable<Varchar>,
        cc_address -> Nullable<Varchar>,
        bcc_address -> Nullable<Varchar>,
        subject -> Nullable<Varchar>,
        content_type -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_leave (party_id, leave_type_id, from_date) {
        party_id -> Varchar,
        leave_type_id -> Varchar,
        empl_leave_reason_type_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        approver_party_id -> Nullable<Varchar>,
        leave_status -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_leave_reason_type (empl_leave_reason_type_id) {
        empl_leave_reason_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_leave_type (leave_type_id) {
        leave_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position (empl_position_id) {
        empl_position_id -> Varchar,
        status_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        budget_id -> Nullable<Varchar>,
        budget_item_seq_id -> Nullable<Varchar>,
        empl_position_type_id -> Nullable<Varchar>,
        estimated_from_date -> Nullable<Timestamptz>,
        estimated_thru_date -> Nullable<Timestamptz>,
        salary_flag -> Nullable<Bpchar>,
        exempt_flag -> Nullable<Bpchar>,
        fulltime_flag -> Nullable<Bpchar>,
        temporary_flag -> Nullable<Bpchar>,
        actual_from_date -> Nullable<Timestamptz>,
        actual_thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position_class_type (empl_position_class_type_id) {
        empl_position_class_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position_fulfillment (empl_position_id, party_id, from_date) {
        empl_position_id -> Varchar,
        party_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position_reporting_struct (empl_position_id_reporting_to, empl_position_id_managed_by, from_date) {
        empl_position_id_reporting_to -> Varchar,
        empl_position_id_managed_by -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        primary_flag -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position_responsibility (empl_position_id, responsibility_type_id, from_date) {
        empl_position_id -> Varchar,
        responsibility_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position_type (empl_position_type_id) {
        empl_position_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position_type_class (empl_position_type_id, empl_position_class_type_id, from_date) {
        empl_position_type_id -> Varchar,
        empl_position_class_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        standard_hours_per_week -> Nullable<Float8>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    empl_position_type_rate_new (empl_position_type_id, rate_type_id, from_date) {
        empl_position_type_id -> Varchar,
        rate_type_id -> Varchar,
        pay_grade_id -> Nullable<Varchar>,
        salary_step_seq_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    employment (role_type_id_from, role_type_id_to, party_id_from, party_id_to, from_date) {
        role_type_id_from -> Varchar,
        role_type_id_to -> Varchar,
        party_id_from -> Varchar,
        party_id_to -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        termination_reason_id -> Nullable<Varchar>,
        termination_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    employment_app (application_id) {
        application_id -> Varchar,
        empl_position_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        employment_app_source_type_id -> Nullable<Varchar>,
        applying_party_id -> Nullable<Varchar>,
        referred_by_party_id -> Nullable<Varchar>,
        application_date -> Nullable<Timestamptz>,
        approver_party_id -> Nullable<Varchar>,
        job_requisition_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    employment_app_source_type (employment_app_source_type_id) {
        employment_app_source_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_audit_log (audit_history_seq_id) {
        audit_history_seq_id -> Varchar,
        changed_entity_name -> Nullable<Varchar>,
        changed_field_name -> Nullable<Varchar>,
        pk_combined_value_text -> Nullable<Varchar>,
        old_value_text -> Nullable<Varchar>,
        new_value_text -> Nullable<Varchar>,
        changed_date -> Nullable<Timestamptz>,
        changed_by_info -> Nullable<Varchar>,
        changed_session_info -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_group (entity_group_id) {
        entity_group_id -> Varchar,
        entity_group_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_group_entry (entity_group_id, entity_or_package) {
        entity_group_id -> Varchar,
        entity_or_package -> Varchar,
        appl_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_key_store (key_name) {
        key_name -> Varchar,
        key_text -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_sync (entity_sync_id) {
        entity_sync_id -> Varchar,
        run_status_id -> Nullable<Varchar>,
        last_successful_synch_time -> Nullable<Timestamptz>,
        last_history_start_date -> Nullable<Timestamptz>,
        pre_offline_synch_time -> Nullable<Timestamptz>,
        offline_sync_split_millis -> Nullable<Numeric>,
        sync_split_millis -> Nullable<Numeric>,
        sync_end_buffer_millis -> Nullable<Numeric>,
        max_running_no_update_millis -> Nullable<Numeric>,
        target_service_name -> Nullable<Varchar>,
        target_delegator_name -> Nullable<Varchar>,
        keep_remove_info_hours -> Nullable<Float8>,
        for_pull_only -> Nullable<Bpchar>,
        for_push_only -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_sync_history (entity_sync_id, start_date) {
        entity_sync_id -> Varchar,
        start_date -> Timestamptz,
        run_status_id -> Nullable<Varchar>,
        beginning_synch_time -> Nullable<Timestamptz>,
        last_successful_synch_time -> Nullable<Timestamptz>,
        last_candidate_end_time -> Nullable<Timestamptz>,
        last_split_start_time -> Nullable<Numeric>,
        to_create_inserted -> Nullable<Numeric>,
        to_create_updated -> Nullable<Numeric>,
        to_create_not_updated -> Nullable<Numeric>,
        to_store_inserted -> Nullable<Numeric>,
        to_store_updated -> Nullable<Numeric>,
        to_store_not_updated -> Nullable<Numeric>,
        to_remove_deleted -> Nullable<Numeric>,
        to_remove_already_deleted -> Nullable<Numeric>,
        total_rows_exported -> Nullable<Numeric>,
        total_rows_to_create -> Nullable<Numeric>,
        total_rows_to_store -> Nullable<Numeric>,
        total_rows_to_remove -> Nullable<Numeric>,
        total_splits -> Nullable<Numeric>,
        total_store_calls -> Nullable<Numeric>,
        running_time_millis -> Nullable<Numeric>,
        per_split_min_millis -> Nullable<Numeric>,
        per_split_max_millis -> Nullable<Numeric>,
        per_split_min_items -> Nullable<Numeric>,
        per_split_max_items -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_sync_include (entity_sync_id, entity_or_package) {
        entity_sync_id -> Varchar,
        entity_or_package -> Varchar,
        appl_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_sync_include_group (entity_sync_id, entity_group_id) {
        entity_sync_id -> Varchar,
        entity_group_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    entity_sync_remove (entity_sync_remove_id) {
        entity_sync_remove_id -> Varchar,
        primary_key_removed -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    enumeration (enum_id) {
        enum_id -> Varchar,
        enum_type_id -> Nullable<Varchar>,
        enum_code -> Nullable<Varchar>,
        sequence_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    enumeration_type (enum_type_id) {
        enum_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    example (example_id) {
        example_id -> Varchar,
        example_type_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        example_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        long_description -> Nullable<Text>,
        comments -> Nullable<Varchar>,
        example_size -> Nullable<Numeric>,
        example_date -> Nullable<Timestamptz>,
        another_date -> Nullable<Timestamptz>,
        another_text -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    example_feature (example_feature_id) {
        example_feature_id -> Varchar,
        feature_source_enum_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    example_feature_appl (example_id, example_feature_id, from_date) {
        example_id -> Varchar,
        example_feature_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        example_feature_appl_type_id -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    example_feature_appl_type (example_feature_appl_type_id) {
        example_feature_appl_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    example_item (example_id, example_item_seq_id) {
        example_id -> Varchar,
        example_item_seq_id -> Varchar,
        description -> Nullable<Varchar>,
        amount -> Nullable<Float8>,
        amount_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    example_status (example_id, status_date) {
        example_id -> Varchar,
        status_date -> Timestamptz,
        status_end_date -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    example_type (example_type_id) {
        example_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    excel_import_history (user_login_id, sequence_num) {
        user_login_id -> Varchar,
        sequence_num -> Numeric,
        file_name -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        thru_reason_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        thread_name -> Nullable<Varchar>,
        log_file_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility (facility_id) {
        facility_id -> Varchar,
        facility_type_id -> Nullable<Varchar>,
        parent_facility_id -> Nullable<Varchar>,
        owner_party_id -> Nullable<Varchar>,
        default_inventory_item_type_id -> Nullable<Varchar>,
        facility_name -> Nullable<Varchar>,
        primary_facility_group_id -> Nullable<Varchar>,
        square_footage -> Nullable<Numeric>,
        facility_size -> Nullable<Numeric>,
        facility_size_uom_id -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        default_days_to_ship -> Nullable<Numeric>,
        opened_date -> Nullable<Timestamptz>,
        closed_date -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        default_dimension_uom_id -> Nullable<Varchar>,
        default_weight_uom_id -> Nullable<Varchar>,
        geo_point_id -> Nullable<Varchar>,
        facility_level -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_assoc_type (facility_assoc_type_id) {
        facility_assoc_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_attribute (facility_id, attr_name) {
        facility_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_calendar (facility_id, calendar_id, facility_calendar_type_id, from_date) {
        facility_id -> Varchar,
        calendar_id -> Varchar,
        facility_calendar_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_calendar_type (facility_calendar_type_id) {
        facility_calendar_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_carrier_shipment (facility_id, party_id, role_type_id, shipment_method_type_id) {
        facility_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        shipment_method_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_contact_mech (facility_id, contact_mech_id, from_date) {
        facility_id -> Varchar,
        contact_mech_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        extension -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_contact_mech_purpose (facility_id, contact_mech_id, contact_mech_purpose_type_id, from_date) {
        facility_id -> Varchar,
        contact_mech_id -> Varchar,
        contact_mech_purpose_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_content (facility_id, content_id, from_date) {
        facility_id -> Varchar,
        content_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_group (facility_group_id) {
        facility_group_id -> Varchar,
        facility_group_type_id -> Nullable<Varchar>,
        primary_parent_group_id -> Nullable<Varchar>,
        facility_group_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_group_member (facility_id, facility_group_id, from_date) {
        facility_id -> Varchar,
        facility_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_group_role (facility_group_id, party_id, role_type_id) {
        facility_group_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_group_rollup (facility_group_id, parent_facility_group_id, from_date) {
        facility_group_id -> Varchar,
        parent_facility_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_group_type (facility_group_type_id) {
        facility_group_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_location (facility_id, location_seq_id) {
        facility_id -> Varchar,
        location_seq_id -> Varchar,
        location_type_enum_id -> Nullable<Varchar>,
        area_id -> Nullable<Varchar>,
        aisle_id -> Nullable<Varchar>,
        section_id -> Nullable<Varchar>,
        level_id -> Nullable<Varchar>,
        position_id -> Nullable<Varchar>,
        geo_point_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_location_geo_point (facility_id, location_seq_id, geo_point_id, from_date) {
        facility_id -> Varchar,
        location_seq_id -> Varchar,
        geo_point_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_party (facility_id, party_id, role_type_id, from_date) {
        facility_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_type (facility_type_id) {
        facility_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    facility_type_attr (facility_type_id, attr_name) {
        facility_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    file_extension (file_extension_id) {
        file_extension_id -> Varchar,
        mime_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account (fin_account_id) {
        fin_account_id -> Varchar,
        fin_account_type_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        fin_account_name -> Nullable<Varchar>,
        fin_account_code -> Nullable<Varchar>,
        fin_account_pin -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        organization_party_id -> Nullable<Varchar>,
        owner_party_id -> Nullable<Varchar>,
        post_to_gl_account_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        is_refundable -> Nullable<Bpchar>,
        replenish_payment_id -> Nullable<Varchar>,
        replenish_level -> Nullable<Numeric>,
        actual_balance -> Nullable<Numeric>,
        available_balance -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_attribute (fin_account_id, attr_name) {
        fin_account_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_auth (fin_account_auth_id) {
        fin_account_auth_id -> Varchar,
        fin_account_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        authorization_date -> Nullable<Timestamptz>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_role (fin_account_id, party_id, role_type_id, from_date) {
        fin_account_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_status (fin_account_id, status_id, status_date) {
        fin_account_id -> Varchar,
        status_id -> Varchar,
        status_date -> Timestamptz,
        status_end_date -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_trans (fin_account_trans_id) {
        fin_account_trans_id -> Varchar,
        fin_account_trans_type_id -> Nullable<Varchar>,
        fin_account_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        gl_reconciliation_id -> Nullable<Varchar>,
        transaction_date -> Nullable<Timestamptz>,
        entry_date -> Nullable<Timestamptz>,
        amount -> Nullable<Numeric>,
        payment_id -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        performed_by_party_id -> Nullable<Varchar>,
        reason_enum_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_trans_attribute (fin_account_trans_id, attr_name) {
        fin_account_trans_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_trans_type (fin_account_trans_type_id) {
        fin_account_trans_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_trans_type_attr (fin_account_trans_type_id, attr_name) {
        fin_account_trans_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_type (fin_account_type_id) {
        fin_account_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        replenish_enum_id -> Nullable<Varchar>,
        is_refundable -> Nullable<Bpchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_type_attr (fin_account_type_id, attr_name) {
        fin_account_type_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fin_account_type_gl_account (fin_account_type_id, organization_party_id) {
        fin_account_type_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset (fixed_asset_id) {
        fixed_asset_id -> Varchar,
        fixed_asset_type_id -> Nullable<Varchar>,
        parent_fixed_asset_id -> Nullable<Varchar>,
        instance_of_product_id -> Nullable<Varchar>,
        class_enum_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        fixed_asset_name -> Nullable<Varchar>,
        acquire_order_id -> Nullable<Varchar>,
        acquire_order_item_seq_id -> Nullable<Varchar>,
        date_acquired -> Nullable<Timestamptz>,
        date_last_serviced -> Nullable<Timestamptz>,
        date_next_service -> Nullable<Timestamptz>,
        expected_end_of_life -> Nullable<Date>,
        actual_end_of_life -> Nullable<Date>,
        production_capacity -> Nullable<Numeric>,
        uom_id -> Nullable<Varchar>,
        calendar_id -> Nullable<Varchar>,
        serial_number -> Nullable<Varchar>,
        located_at_facility_id -> Nullable<Varchar>,
        located_at_location_seq_id -> Nullable<Varchar>,
        salvage_value -> Nullable<Numeric>,
        depreciation -> Nullable<Numeric>,
        purchase_cost -> Nullable<Numeric>,
        purchase_cost_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_attribute (fixed_asset_id, attr_name) {
        fixed_asset_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_dep_method (depreciation_custom_method_id, fixed_asset_id) {
        depreciation_custom_method_id -> Varchar,
        fixed_asset_id -> Varchar,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_geo_point (fixed_asset_id, geo_point_id, from_date) {
        fixed_asset_id -> Varchar,
        geo_point_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_ident (fixed_asset_id, fixed_asset_ident_type_id) {
        fixed_asset_id -> Varchar,
        fixed_asset_ident_type_id -> Varchar,
        id_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_ident_type (fixed_asset_ident_type_id) {
        fixed_asset_ident_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_maint (fixed_asset_id, maint_hist_seq_id) {
        fixed_asset_id -> Varchar,
        maint_hist_seq_id -> Varchar,
        status_id -> Nullable<Varchar>,
        product_maint_type_id -> Nullable<Varchar>,
        product_maint_seq_id -> Nullable<Varchar>,
        schedule_work_effort_id -> Nullable<Varchar>,
        interval_quantity -> Nullable<Numeric>,
        interval_uom_id -> Nullable<Varchar>,
        interval_meter_type_id -> Nullable<Varchar>,
        purchase_order_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_maint_order (fixed_asset_id, maint_hist_seq_id, order_id, order_item_seq_id) {
        fixed_asset_id -> Varchar,
        maint_hist_seq_id -> Varchar,
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_meter (fixed_asset_id, product_meter_type_id, reading_date) {
        fixed_asset_id -> Varchar,
        product_meter_type_id -> Varchar,
        reading_date -> Timestamptz,
        meter_value -> Nullable<Numeric>,
        reading_reason_enum_id -> Nullable<Varchar>,
        maint_hist_seq_id -> Nullable<Varchar>,
        work_effort_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_product (fixed_asset_id, product_id, fixed_asset_product_type_id, from_date) {
        fixed_asset_id -> Varchar,
        product_id -> Varchar,
        fixed_asset_product_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        quantity -> Nullable<Numeric>,
        quantity_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_product_type (fixed_asset_product_type_id) {
        fixed_asset_product_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_registration (fixed_asset_id, from_date) {
        fixed_asset_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        registration_date -> Nullable<Timestamptz>,
        gov_agency_party_id -> Nullable<Varchar>,
        registration_number -> Nullable<Varchar>,
        license_number -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_std_cost (fixed_asset_id, fixed_asset_std_cost_type_id, from_date) {
        fixed_asset_id -> Varchar,
        fixed_asset_std_cost_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        amount_uom_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_std_cost_type (fixed_asset_std_cost_type_id) {
        fixed_asset_std_cost_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_type (fixed_asset_type_id) {
        fixed_asset_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_type_attr (fixed_asset_type_id, attr_name) {
        fixed_asset_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    fixed_asset_type_gl_account (fixed_asset_type_id, fixed_asset_id, organization_party_id) {
        fixed_asset_type_id -> Varchar,
        fixed_asset_id -> Varchar,
        organization_party_id -> Varchar,
        asset_gl_account_id -> Nullable<Varchar>,
        acc_dep_gl_account_id -> Nullable<Varchar>,
        dep_gl_account_id -> Nullable<Varchar>,
        profit_gl_account_id -> Nullable<Varchar>,
        loss_gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    ftp_address (contact_mech_id) {
        contact_mech_id -> Varchar,
        hostname -> Nullable<Varchar>,
        port -> Nullable<Numeric>,
        username -> Nullable<Varchar>,
        ftp_password -> Nullable<Varchar>,
        binary_transfer -> Nullable<Bpchar>,
        file_path -> Nullable<Varchar>,
        zip_file -> Nullable<Bpchar>,
        passive_mode -> Nullable<Bpchar>,
        default_timeout -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    geo (geo_id) {
        geo_id -> Varchar,
        geo_type_id -> Nullable<Varchar>,
        geo_name -> Nullable<Varchar>,
        geo_code -> Nullable<Varchar>,
        geo_sec_code -> Nullable<Varchar>,
        abbreviation -> Nullable<Varchar>,
        well_known_text -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    geo_assoc (geo_id, geo_id_to) {
        geo_id -> Varchar,
        geo_id_to -> Varchar,
        geo_assoc_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    geo_assoc_type (geo_assoc_type_id) {
        geo_assoc_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    geo_point (geo_point_id) {
        geo_point_id -> Varchar,
        geo_point_type_enum_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        latitude -> Varchar,
        longitude -> Varchar,
        elevation -> Nullable<Numeric>,
        elevation_uom_id -> Nullable<Varchar>,
        information -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    geo_type (geo_type_id) {
        geo_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gift_card (payment_method_id) {
        payment_method_id -> Varchar,
        card_number -> Nullable<Varchar>,
        pin_number -> Nullable<Varchar>,
        expire_date -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gift_card_fulfillment (fulfillment_id) {
        fulfillment_id -> Varchar,
        type_enum_id -> Nullable<Varchar>,
        merchant_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        survey_response_id -> Nullable<Varchar>,
        card_number -> Nullable<Varchar>,
        pin_number -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        response_code -> Nullable<Varchar>,
        reference_num -> Nullable<Varchar>,
        auth_code -> Nullable<Varchar>,
        fulfillment_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    git_hub_user (git_hub_user_id) {
        git_hub_user_id -> Varchar,
        product_store_id -> Nullable<Varchar>,
        env_prefix -> Nullable<Varchar>,
        token_type -> Nullable<Varchar>,
        access_token -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account (gl_account_id) {
        gl_account_id -> Varchar,
        gl_account_type_id -> Nullable<Varchar>,
        gl_account_class_id -> Nullable<Varchar>,
        gl_resource_type_id -> Nullable<Varchar>,
        gl_xbrl_class_id -> Nullable<Varchar>,
        parent_gl_account_id -> Nullable<Varchar>,
        account_code -> Nullable<Varchar>,
        account_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        external_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_category (gl_account_category_id) {
        gl_account_category_id -> Varchar,
        gl_account_category_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_category_member (gl_account_id, gl_account_category_id, from_date) {
        gl_account_id -> Varchar,
        gl_account_category_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        amount_percentage -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_category_type (gl_account_category_type_id) {
        gl_account_category_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_class (gl_account_class_id) {
        gl_account_class_id -> Varchar,
        parent_class_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        is_asset_class -> Nullable<Bpchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_group (gl_account_group_id) {
        gl_account_group_id -> Varchar,
        gl_account_group_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_group_member (gl_account_id, gl_account_group_type_id) {
        gl_account_id -> Varchar,
        gl_account_group_type_id -> Varchar,
        gl_account_group_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_group_type (gl_account_group_type_id) {
        gl_account_group_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_history (gl_account_id, organization_party_id, custom_time_period_id) {
        gl_account_id -> Varchar,
        organization_party_id -> Varchar,
        custom_time_period_id -> Varchar,
        opening_balance -> Nullable<Numeric>,
        posted_debits -> Nullable<Numeric>,
        posted_credits -> Nullable<Numeric>,
        ending_balance -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_organization (gl_account_id, organization_party_id) {
        gl_account_id -> Varchar,
        organization_party_id -> Varchar,
        role_type_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_role (gl_account_id, party_id, role_type_id, from_date) {
        gl_account_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_type (gl_account_type_id) {
        gl_account_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_account_type_default (gl_account_type_id, organization_party_id) {
        gl_account_type_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_budget_xref (gl_account_id, budget_item_type_id, from_date) {
        gl_account_id -> Varchar,
        budget_item_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        allocation_percentage -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_fiscal_type (gl_fiscal_type_id) {
        gl_fiscal_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_journal (gl_journal_id) {
        gl_journal_id -> Varchar,
        gl_journal_name -> Nullable<Varchar>,
        organization_party_id -> Nullable<Varchar>,
        is_posted -> Nullable<Bpchar>,
        posted_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_reconciliation (gl_reconciliation_id) {
        gl_reconciliation_id -> Varchar,
        gl_reconciliation_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        gl_account_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        organization_party_id -> Nullable<Varchar>,
        reconciled_balance -> Nullable<Numeric>,
        opening_balance -> Nullable<Numeric>,
        reconciled_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_reconciliation_entry (gl_reconciliation_id, acctg_trans_id, acctg_trans_entry_seq_id) {
        gl_reconciliation_id -> Varchar,
        acctg_trans_id -> Varchar,
        acctg_trans_entry_seq_id -> Varchar,
        reconciled_amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_resource_type (gl_resource_type_id) {
        gl_resource_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    gl_xbrl_class (gl_xbrl_class_id) {
        gl_xbrl_class_id -> Varchar,
        parent_gl_xbrl_class_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    good_identification (good_identification_type_id, product_id) {
        good_identification_type_id -> Varchar,
        product_id -> Varchar,
        id_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    good_identification_type (good_identification_type_id) {
        good_identification_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    image_data_resource (data_resource_id) {
        data_resource_id -> Varchar,
        image_data -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item (inventory_item_id) {
        inventory_item_id -> Varchar,
        inventory_item_type_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        owner_party_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        datetime_received -> Nullable<Timestamptz>,
        datetime_manufactured -> Nullable<Timestamptz>,
        expire_date -> Nullable<Timestamptz>,
        facility_id -> Nullable<Varchar>,
        container_id -> Nullable<Varchar>,
        lot_id -> Nullable<Varchar>,
        uom_id -> Nullable<Varchar>,
        bin_number -> Nullable<Varchar>,
        location_seq_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        quantity_on_hand_total -> Nullable<Numeric>,
        available_to_promise_total -> Nullable<Numeric>,
        accounting_quantity_total -> Nullable<Numeric>,
        quantity_on_hand -> Nullable<Numeric>,
        available_to_promise -> Nullable<Numeric>,
        serial_number -> Nullable<Varchar>,
        soft_identifier -> Nullable<Varchar>,
        activation_number -> Nullable<Varchar>,
        activation_valid_thru -> Nullable<Timestamptz>,
        unit_cost -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_attribute (inventory_item_id, attr_name) {
        inventory_item_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_detail (inventory_item_id, inventory_item_detail_seq_id) {
        inventory_item_id -> Varchar,
        inventory_item_detail_seq_id -> Varchar,
        effective_date -> Nullable<Timestamptz>,
        quantity_on_hand_diff -> Nullable<Numeric>,
        available_to_promise_diff -> Nullable<Numeric>,
        accounting_quantity_diff -> Nullable<Numeric>,
        unit_cost -> Nullable<Numeric>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        ship_group_seq_id -> Nullable<Varchar>,
        shipment_id -> Nullable<Varchar>,
        shipment_item_seq_id -> Nullable<Varchar>,
        return_id -> Nullable<Varchar>,
        return_item_seq_id -> Nullable<Varchar>,
        work_effort_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        maint_hist_seq_id -> Nullable<Varchar>,
        item_issuance_id -> Nullable<Varchar>,
        receipt_id -> Nullable<Varchar>,
        physical_inventory_id -> Nullable<Varchar>,
        reason_enum_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_label (inventory_item_label_id) {
        inventory_item_label_id -> Varchar,
        inventory_item_label_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_label_appl (inventory_item_id, inventory_item_label_type_id) {
        inventory_item_id -> Varchar,
        inventory_item_label_type_id -> Varchar,
        inventory_item_label_id -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_label_type (inventory_item_label_type_id) {
        inventory_item_label_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_status (inventory_item_id, status_id, status_datetime) {
        inventory_item_id -> Varchar,
        status_id -> Varchar,
        status_datetime -> Timestamptz,
        status_end_datetime -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Varchar>,
        owner_party_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_temp_res (visit_id, product_id, product_store_id) {
        visit_id -> Varchar,
        product_id -> Varchar,
        product_store_id -> Varchar,
        quantity -> Nullable<Numeric>,
        reserved_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_type (inventory_item_type_id) {
        inventory_item_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_type_attr (inventory_item_type_id, attr_name) {
        inventory_item_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_item_variance (inventory_item_id, physical_inventory_id) {
        inventory_item_id -> Varchar,
        physical_inventory_id -> Varchar,
        variance_reason_id -> Nullable<Varchar>,
        available_to_promise_var -> Nullable<Numeric>,
        quantity_on_hand_var -> Nullable<Numeric>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    inventory_transfer (inventory_transfer_id) {
        inventory_transfer_id -> Varchar,
        status_id -> Nullable<Varchar>,
        inventory_item_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        location_seq_id -> Nullable<Varchar>,
        container_id -> Nullable<Varchar>,
        facility_id_to -> Nullable<Varchar>,
        location_seq_id_to -> Nullable<Varchar>,
        container_id_to -> Nullable<Varchar>,
        item_issuance_id -> Nullable<Varchar>,
        send_date -> Nullable<Timestamptz>,
        receive_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice (invoice_id) {
        invoice_id -> Varchar,
        invoice_type_id -> Nullable<Varchar>,
        party_id_from -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        billing_account_id -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        invoice_date -> Nullable<Timestamptz>,
        due_date -> Nullable<Timestamptz>,
        paid_date -> Nullable<Timestamptz>,
        invoice_message -> Nullable<Varchar>,
        reference_number -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        recurrence_info_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_attribute (invoice_id, attr_name) {
        invoice_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_contact_mech (invoice_id, contact_mech_purpose_type_id, contact_mech_id) {
        invoice_id -> Varchar,
        contact_mech_purpose_type_id -> Varchar,
        contact_mech_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_content (content_id, invoice_id, invoice_content_type_id, from_date) {
        invoice_id -> Varchar,
        invoice_content_type_id -> Varchar,
        content_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_content_type (invoice_content_type_id) {
        invoice_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item (invoice_id, invoice_item_seq_id) {
        invoice_id -> Varchar,
        invoice_item_seq_id -> Varchar,
        invoice_item_type_id -> Nullable<Varchar>,
        override_gl_account_id -> Nullable<Varchar>,
        override_org_party_id -> Nullable<Varchar>,
        inventory_item_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        product_feature_id -> Nullable<Varchar>,
        parent_invoice_id -> Nullable<Varchar>,
        parent_invoice_item_seq_id -> Nullable<Varchar>,
        uom_id -> Nullable<Varchar>,
        taxable_flag -> Nullable<Bpchar>,
        quantity -> Nullable<Numeric>,
        amount -> Nullable<Numeric>,
        description -> Nullable<Varchar>,
        tax_auth_party_id -> Nullable<Varchar>,
        tax_auth_geo_id -> Nullable<Varchar>,
        tax_authority_rate_seq_id -> Nullable<Varchar>,
        sales_opportunity_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item_assoc (invoice_id_from, invoice_item_seq_id_from, invoice_id_to, invoice_item_seq_id_to, invoice_item_assoc_type_id, from_date) {
        invoice_id_from -> Varchar,
        invoice_item_seq_id_from -> Varchar,
        invoice_id_to -> Varchar,
        invoice_item_seq_id_to -> Varchar,
        invoice_item_assoc_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        party_id_from -> Nullable<Varchar>,
        party_id_to -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item_assoc_type (invoice_item_assoc_type_id) {
        invoice_item_assoc_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item_attribute (invoice_id, invoice_item_seq_id, attr_name) {
        invoice_id -> Varchar,
        invoice_item_seq_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item_type (invoice_item_type_id) {
        invoice_item_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        default_gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item_type_attr (invoice_item_type_id, attr_name) {
        invoice_item_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item_type_gl_account (invoice_item_type_id, organization_party_id) {
        invoice_item_type_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_item_type_map (invoice_item_map_key, invoice_type_id) {
        invoice_item_map_key -> Varchar,
        invoice_type_id -> Varchar,
        invoice_item_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_note (invoice_id, note_id) {
        invoice_id -> Varchar,
        note_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_role (invoice_id, party_id, role_type_id) {
        invoice_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        datetime_performed -> Nullable<Timestamptz>,
        percentage -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_status (status_id, invoice_id, status_date) {
        status_id -> Varchar,
        invoice_id -> Varchar,
        status_date -> Timestamptz,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_term (invoice_term_id) {
        invoice_term_id -> Varchar,
        term_type_id -> Nullable<Varchar>,
        invoice_id -> Nullable<Varchar>,
        invoice_item_seq_id -> Nullable<Varchar>,
        term_value -> Nullable<Numeric>,
        term_days -> Nullable<Numeric>,
        text_value -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_term_attribute (invoice_term_id, attr_name) {
        invoice_term_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_type (invoice_type_id) {
        invoice_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    invoice_type_attr (invoice_type_id, attr_name) {
        invoice_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    item_issuance (item_issuance_id) {
        item_issuance_id -> Varchar,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        ship_group_seq_id -> Nullable<Varchar>,
        inventory_item_id -> Nullable<Varchar>,
        shipment_id -> Nullable<Varchar>,
        shipment_item_seq_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        maint_hist_seq_id -> Nullable<Varchar>,
        issued_date_time -> Nullable<Timestamptz>,
        issued_by_user_login_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        cancel_quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    item_issuance_role (item_issuance_id, party_id, role_type_id) {
        item_issuance_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    java_resource (resource_name) {
        resource_name -> Varchar,
        resource_value -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    job_interview (job_interview_id) {
        job_interview_id -> Varchar,
        job_interviewee_party_id -> Nullable<Varchar>,
        job_requisition_id -> Nullable<Varchar>,
        job_interviewer_party_id -> Nullable<Varchar>,
        job_interview_type_id -> Nullable<Varchar>,
        grade_secured_enum_id -> Nullable<Varchar>,
        job_interview_result -> Nullable<Varchar>,
        job_interview_date -> Nullable<Date>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    job_interview_type (job_interview_type_id) {
        job_interview_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    job_manager_lock (instance_id, from_date) {
        instance_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        reason_enum_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    job_requisition (job_requisition_id) {
        job_requisition_id -> Varchar,
        duration_months -> Nullable<Numeric>,
        age -> Nullable<Numeric>,
        gender -> Nullable<Bpchar>,
        experience_months -> Nullable<Numeric>,
        experience_years -> Nullable<Numeric>,
        qualification -> Nullable<Varchar>,
        job_location -> Nullable<Varchar>,
        skill_type_id -> Nullable<Varchar>,
        no_of_resources -> Nullable<Numeric>,
        job_posting_type_enum_id -> Nullable<Varchar>,
        job_requisition_date -> Nullable<Date>,
        exam_type_enum_id -> Nullable<Varchar>,
        required_on_date -> Nullable<Date>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    job_sandbox (job_id) {
        job_id -> Varchar,
        job_name -> Nullable<Varchar>,
        run_time -> Nullable<Timestamptz>,
        priority -> Nullable<Numeric>,
        pool_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        parent_job_id -> Nullable<Varchar>,
        previous_job_id -> Nullable<Varchar>,
        service_name -> Nullable<Varchar>,
        loader_name -> Nullable<Varchar>,
        max_retry -> Nullable<Numeric>,
        current_retry_count -> Nullable<Numeric>,
        auth_user_login_id -> Nullable<Varchar>,
        run_as_user -> Nullable<Varchar>,
        runtime_data_id -> Nullable<Varchar>,
        recurrence_info_id -> Nullable<Varchar>,
        temp_expr_id -> Nullable<Varchar>,
        current_recurrence_count -> Nullable<Numeric>,
        max_recurrence_count -> Nullable<Numeric>,
        run_by_instance_id -> Nullable<Varchar>,
        start_date_time -> Nullable<Timestamptz>,
        finish_date_time -> Nullable<Timestamptz>,
        cancel_date_time -> Nullable<Timestamptz>,
        job_result -> Nullable<Varchar>,
        recurrence_time_zone -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    keyword_thesaurus (entered_keyword, alternate_keyword) {
        entered_keyword -> Varchar,
        alternate_keyword -> Varchar,
        relationship_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    linked_in_user (linked_in_user_id) {
        linked_in_user_id -> Varchar,
        product_store_id -> Nullable<Varchar>,
        env_prefix -> Nullable<Varchar>,
        access_token -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    lot (lot_id) {
        lot_id -> Varchar,
        creation_date -> Nullable<Timestamptz>,
        quantity -> Nullable<Numeric>,
        expiration_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    market_interest (product_category_id, party_classification_group_id, from_date) {
        product_category_id -> Varchar,
        party_classification_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    marketing_campaign (marketing_campaign_id) {
        marketing_campaign_id -> Varchar,
        parent_campaign_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        campaign_name -> Nullable<Varchar>,
        campaign_summary -> Nullable<Text>,
        budgeted_cost -> Nullable<Numeric>,
        actual_cost -> Nullable<Numeric>,
        estimated_cost -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        is_active -> Nullable<Bpchar>,
        converted_leads -> Nullable<Varchar>,
        expected_response_percent -> Nullable<Float8>,
        expected_revenue -> Nullable<Numeric>,
        num_sent -> Nullable<Numeric>,
        start_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    marketing_campaign_note (marketing_campaign_id, note_id) {
        marketing_campaign_id -> Varchar,
        note_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    marketing_campaign_price (marketing_campaign_id, product_price_rule_id, from_date) {
        marketing_campaign_id -> Varchar,
        product_price_rule_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    marketing_campaign_promo (marketing_campaign_id, product_promo_id, from_date) {
        marketing_campaign_id -> Varchar,
        product_promo_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    marketing_campaign_role (marketing_campaign_id, party_id, role_type_id, from_date) {
        marketing_campaign_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    meta_data_predicate (meta_data_predicate_id) {
        meta_data_predicate_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    mime_type (mime_type_id) {
        mime_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    mime_type_html_template (mime_type_id) {
        mime_type_id -> Varchar,
        template_location -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    mrp_event (mrp_id, product_id, event_date, mrp_event_type_id) {
        mrp_id -> Varchar,
        product_id -> Varchar,
        event_date -> Timestamptz,
        mrp_event_type_id -> Varchar,
        facility_id -> Nullable<Varchar>,
        quantity -> Nullable<Float8>,
        event_name -> Nullable<Text>,
        is_late -> Nullable<Bpchar>,
        facility_id_to -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    mrp_event_type (mrp_event_type_id) {
        mrp_event_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    msg91_gateway_config (telecom_gateway_config_id) {
        telecom_gateway_config_id -> Varchar,
        api_url -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        route -> Nullable<Varchar>,
        authkey -> Nullable<Varchar>,
        sender -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    need_type (need_type_id) {
        need_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    note_data (note_id) {
        note_id -> Varchar,
        note_name -> Nullable<Varchar>,
        note_info -> Nullable<Text>,
        note_date_time -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        note_party -> Nullable<Varchar>,
        more_info_url -> Nullable<Varchar>,
        more_info_item_id -> Nullable<Varchar>,
        more_info_item_name -> Nullable<Varchar>,
    }
}

table! {
    o_auth2_git_hub (product_store_id, from_date) {
        product_store_id -> Varchar,
        client_id -> Nullable<Varchar>,
        client_secret -> Nullable<Varchar>,
        return_url -> Nullable<Varchar>,
        local_redirect_uri -> Nullable<Varchar>,
        icon_url -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    o_auth2_linked_in (product_store_id, from_date) {
        product_store_id -> Varchar,
        api_key -> Nullable<Varchar>,
        secret_key -> Nullable<Varchar>,
        live_return_url -> Nullable<Varchar>,
        test_return_url -> Nullable<Varchar>,
        local_redirect_uri -> Nullable<Varchar>,
        icon_url -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_adjustment (order_adjustment_id) {
        order_adjustment_id -> Varchar,
        order_adjustment_type_id -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        ship_group_seq_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        recurring_amount -> Nullable<Numeric>,
        amount_already_included -> Nullable<Numeric>,
        product_promo_id -> Nullable<Varchar>,
        product_promo_rule_id -> Nullable<Varchar>,
        product_promo_action_seq_id -> Nullable<Varchar>,
        product_feature_id -> Nullable<Varchar>,
        corresponding_product_id -> Nullable<Varchar>,
        tax_authority_rate_seq_id -> Nullable<Varchar>,
        source_reference_id -> Nullable<Varchar>,
        source_percentage -> Nullable<Numeric>,
        customer_reference_id -> Nullable<Varchar>,
        primary_geo_id -> Nullable<Varchar>,
        secondary_geo_id -> Nullable<Varchar>,
        exempt_amount -> Nullable<Numeric>,
        tax_auth_geo_id -> Nullable<Varchar>,
        tax_auth_party_id -> Nullable<Varchar>,
        override_gl_account_id -> Nullable<Varchar>,
        include_in_tax -> Nullable<Bpchar>,
        include_in_shipping -> Nullable<Bpchar>,
        is_manual -> Nullable<Bpchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        original_adjustment_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_adjustment_attribute (order_adjustment_id, attr_name) {
        order_adjustment_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_adjustment_billing (order_adjustment_id, invoice_id, invoice_item_seq_id) {
        order_adjustment_id -> Varchar,
        invoice_id -> Varchar,
        invoice_item_seq_id -> Varchar,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_adjustment_type (order_adjustment_type_id) {
        order_adjustment_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_adjustment_type_attr (order_adjustment_type_id, attr_name) {
        order_adjustment_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_attribute (order_id, attr_name) {
        order_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_blacklist (blacklist_string, order_blacklist_type_id) {
        blacklist_string -> Varchar,
        order_blacklist_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_blacklist_type (order_blacklist_type_id) {
        order_blacklist_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_contact_mech (order_id, contact_mech_purpose_type_id, contact_mech_id) {
        order_id -> Varchar,
        contact_mech_purpose_type_id -> Varchar,
        contact_mech_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_content (content_id, order_id, order_item_seq_id, order_content_type_id, from_date) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        content_id -> Varchar,
        order_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_content_type (order_content_type_id) {
        order_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_delivery_schedule (order_id, order_item_seq_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        estimated_ready_date -> Nullable<Timestamptz>,
        cartons -> Nullable<Numeric>,
        skids_pallets -> Nullable<Numeric>,
        units_pieces -> Nullable<Numeric>,
        total_cubic_size -> Nullable<Numeric>,
        total_cubic_uom_id -> Nullable<Varchar>,
        total_weight -> Nullable<Numeric>,
        total_weight_uom_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_denylist (denylist_string, order_denylist_type_id) {
        denylist_string -> Varchar,
        order_denylist_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_denylist_type (order_denylist_type_id) {
        order_denylist_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_header (order_id) {
        order_id -> Varchar,
        order_type_id -> Nullable<Varchar>,
        order_name -> Nullable<Varchar>,
        external_id -> Nullable<Varchar>,
        sales_channel_enum_id -> Nullable<Varchar>,
        order_date -> Nullable<Timestamptz>,
        priority -> Nullable<Bpchar>,
        entry_date -> Nullable<Timestamptz>,
        pick_sheet_printed_date -> Nullable<Timestamptz>,
        visit_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        created_by -> Nullable<Varchar>,
        first_attempt_order_id -> Nullable<Varchar>,
        currency_uom -> Nullable<Varchar>,
        sync_status_id -> Nullable<Varchar>,
        billing_account_id -> Nullable<Varchar>,
        origin_facility_id -> Nullable<Varchar>,
        web_site_id -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        agreement_id -> Nullable<Varchar>,
        terminal_id -> Nullable<Varchar>,
        transaction_id -> Nullable<Varchar>,
        auto_order_shopping_list_id -> Nullable<Varchar>,
        needs_inventory_issuance -> Nullable<Bpchar>,
        is_rush_order -> Nullable<Bpchar>,
        internal_code -> Nullable<Varchar>,
        remaining_sub_total -> Nullable<Numeric>,
        grand_total -> Nullable<Numeric>,
        is_viewed -> Nullable<Bpchar>,
        invoice_per_shipment -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_header_note (order_id, note_id) {
        order_id -> Varchar,
        note_id -> Varchar,
        internal_note -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_header_work_effort (order_id, work_effort_id) {
        order_id -> Varchar,
        work_effort_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item (order_id, order_item_seq_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        external_id -> Nullable<Varchar>,
        order_item_type_id -> Nullable<Varchar>,
        order_item_group_seq_id -> Nullable<Varchar>,
        is_item_group_primary -> Nullable<Bpchar>,
        from_inventory_item_id -> Nullable<Varchar>,
        budget_id -> Nullable<Varchar>,
        budget_item_seq_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        supplier_product_id -> Nullable<Varchar>,
        product_feature_id -> Nullable<Varchar>,
        prod_catalog_id -> Nullable<Varchar>,
        product_category_id -> Nullable<Varchar>,
        is_promo -> Nullable<Bpchar>,
        quote_id -> Nullable<Varchar>,
        quote_item_seq_id -> Nullable<Varchar>,
        shopping_list_id -> Nullable<Varchar>,
        shopping_list_item_seq_id -> Nullable<Varchar>,
        subscription_id -> Nullable<Varchar>,
        deployment_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        cancel_quantity -> Nullable<Numeric>,
        selected_amount -> Nullable<Numeric>,
        unit_price -> Nullable<Numeric>,
        unit_list_price -> Nullable<Numeric>,
        unit_average_cost -> Nullable<Numeric>,
        unit_recurring_price -> Nullable<Numeric>,
        is_modified_price -> Nullable<Bpchar>,
        recurring_freq_uom_id -> Nullable<Varchar>,
        item_description -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        corresponding_po_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        sync_status_id -> Nullable<Varchar>,
        estimated_ship_date -> Nullable<Timestamptz>,
        estimated_delivery_date -> Nullable<Timestamptz>,
        auto_cancel_date -> Nullable<Timestamptz>,
        dont_cancel_set_date -> Nullable<Timestamptz>,
        dont_cancel_set_user_login -> Nullable<Varchar>,
        ship_before_date -> Nullable<Timestamptz>,
        ship_after_date -> Nullable<Timestamptz>,
        reserve_after_date -> Nullable<Timestamptz>,
        cancel_back_order_date -> Nullable<Timestamptz>,
        override_gl_account_id -> Nullable<Varchar>,
        sales_opportunity_id -> Nullable<Varchar>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_assoc (order_id, order_item_seq_id, ship_group_seq_id, to_order_id, to_order_item_seq_id, to_ship_group_seq_id, order_item_assoc_type_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        ship_group_seq_id -> Varchar,
        to_order_id -> Varchar,
        to_order_item_seq_id -> Varchar,
        to_ship_group_seq_id -> Varchar,
        order_item_assoc_type_id -> Varchar,
        quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_assoc_type (order_item_assoc_type_id) {
        order_item_assoc_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_attribute (order_id, order_item_seq_id, attr_name) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_billing (order_id, order_item_seq_id, invoice_id, invoice_item_seq_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        invoice_id -> Varchar,
        invoice_item_seq_id -> Varchar,
        item_issuance_id -> Nullable<Varchar>,
        shipment_receipt_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_change (order_item_change_id) {
        order_item_change_id -> Varchar,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        change_type_enum_id -> Nullable<Varchar>,
        change_datetime -> Nullable<Timestamptz>,
        change_user_login -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        cancel_quantity -> Nullable<Numeric>,
        unit_price -> Nullable<Numeric>,
        item_description -> Nullable<Varchar>,
        reason_enum_id -> Nullable<Varchar>,
        change_comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_contact_mech (order_id, order_item_seq_id, contact_mech_purpose_type_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        contact_mech_purpose_type_id -> Varchar,
        contact_mech_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_group (order_id, order_item_group_seq_id) {
        order_id -> Varchar,
        order_item_group_seq_id -> Varchar,
        parent_group_seq_id -> Nullable<Varchar>,
        group_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_group_order (order_id, order_item_seq_id, group_order_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        group_order_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_price_info (order_item_price_info_id) {
        order_item_price_info_id -> Varchar,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        product_price_rule_id -> Nullable<Varchar>,
        product_price_action_seq_id -> Nullable<Varchar>,
        modify_amount -> Nullable<Numeric>,
        description -> Nullable<Varchar>,
        rate_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_role (order_id, order_item_seq_id, party_id, role_type_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_ship_group (order_id, ship_group_seq_id) {
        order_id -> Varchar,
        ship_group_seq_id -> Varchar,
        shipment_method_type_id -> Nullable<Varchar>,
        supplier_party_id -> Nullable<Varchar>,
        supplier_agreement_id -> Nullable<Varchar>,
        vendor_party_id -> Nullable<Varchar>,
        carrier_party_id -> Nullable<Varchar>,
        carrier_role_type_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        telecom_contact_mech_id -> Nullable<Varchar>,
        tracking_number -> Nullable<Varchar>,
        shipping_instructions -> Nullable<Varchar>,
        may_split -> Nullable<Bpchar>,
        gift_message -> Nullable<Varchar>,
        is_gift -> Nullable<Bpchar>,
        ship_after_date -> Nullable<Timestamptz>,
        ship_by_date -> Nullable<Timestamptz>,
        estimated_ship_date -> Nullable<Timestamptz>,
        estimated_delivery_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_ship_group_assoc (order_id, order_item_seq_id, ship_group_seq_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        ship_group_seq_id -> Varchar,
        quantity -> Nullable<Numeric>,
        cancel_quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_ship_grp_inv_res (order_id, ship_group_seq_id, order_item_seq_id, inventory_item_id) {
        order_id -> Varchar,
        ship_group_seq_id -> Varchar,
        order_item_seq_id -> Varchar,
        inventory_item_id -> Varchar,
        reserve_order_enum_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        quantity_not_available -> Nullable<Numeric>,
        reserved_datetime -> Nullable<Timestamptz>,
        created_datetime -> Nullable<Timestamptz>,
        promised_datetime -> Nullable<Timestamptz>,
        current_promised_date -> Nullable<Timestamptz>,
        priority -> Nullable<Bpchar>,
        sequence_id -> Nullable<Numeric>,
        pick_start_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_type (order_item_type_id) {
        order_item_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_item_type_attr (order_item_type_id, attr_name) {
        order_item_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_notification (order_notification_id) {
        order_notification_id -> Varchar,
        order_id -> Nullable<Varchar>,
        email_type -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        notification_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_payment_preference (order_payment_preference_id) {
        order_payment_preference_id -> Varchar,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        ship_group_seq_id -> Nullable<Varchar>,
        product_price_purpose_id -> Nullable<Varchar>,
        payment_method_type_id -> Nullable<Varchar>,
        payment_method_id -> Nullable<Varchar>,
        fin_account_id -> Nullable<Varchar>,
        security_code -> Nullable<Varchar>,
        track2 -> Nullable<Varchar>,
        present_flag -> Nullable<Bpchar>,
        swiped_flag -> Nullable<Bpchar>,
        overflow_flag -> Nullable<Bpchar>,
        max_amount -> Nullable<Numeric>,
        process_attempt -> Nullable<Numeric>,
        billing_postal_code -> Nullable<Varchar>,
        manual_auth_code -> Nullable<Varchar>,
        manual_ref_num -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        needs_nsf_retry -> Nullable<Bpchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_product_promo_code (order_id, product_promo_code_id) {
        order_id -> Varchar,
        product_promo_code_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_requirement_commitment (order_id, order_item_seq_id, requirement_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        requirement_id -> Varchar,
        quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_role (order_id, party_id, role_type_id) {
        order_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_shipment (order_id, order_item_seq_id, ship_group_seq_id, shipment_id, shipment_item_seq_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        ship_group_seq_id -> Varchar,
        shipment_id -> Varchar,
        shipment_item_seq_id -> Varchar,
        quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_status (order_status_id) {
        order_status_id -> Varchar,
        status_id -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        order_payment_preference_id -> Nullable<Varchar>,
        status_datetime -> Nullable<Timestamptz>,
        status_user_login -> Nullable<Varchar>,
        change_reason -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_summary_entry (entry_date, product_id, facility_id) {
        entry_date -> Date,
        product_id -> Varchar,
        facility_id -> Varchar,
        total_quantity -> Nullable<Numeric>,
        gross_sales -> Nullable<Numeric>,
        product_cost -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_term (term_type_id, order_id, order_item_seq_id) {
        term_type_id -> Varchar,
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        term_value -> Nullable<Numeric>,
        term_days -> Nullable<Numeric>,
        text_value -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_term_attribute (term_type_id, order_id, order_item_seq_id, attr_name) {
        term_type_id -> Varchar,
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_type (order_type_id) {
        order_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    order_type_attr (order_type_id, attr_name) {
        order_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    other_data_resource (data_resource_id) {
        data_resource_id -> Varchar,
        data_resource_content -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party (party_id) {
        party_id -> Varchar,
        party_type_id -> Nullable<Varchar>,
        external_id -> Nullable<Varchar>,
        preferred_currency_uom_id -> Nullable<Varchar>,
        description -> Nullable<Text>,
        status_id -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        is_unread -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_acctg_preference (party_id) {
        party_id -> Varchar,
        fiscal_year_start_month -> Nullable<Numeric>,
        fiscal_year_start_day -> Nullable<Numeric>,
        tax_form_id -> Nullable<Varchar>,
        cogs_method_id -> Nullable<Varchar>,
        base_currency_uom_id -> Nullable<Varchar>,
        invoice_seq_cust_meth_id -> Nullable<Varchar>,
        invoice_id_prefix -> Nullable<Varchar>,
        last_invoice_number -> Nullable<Numeric>,
        last_invoice_restart_date -> Nullable<Timestamptz>,
        use_invoice_id_for_returns -> Nullable<Bpchar>,
        quote_seq_cust_meth_id -> Nullable<Varchar>,
        quote_id_prefix -> Nullable<Varchar>,
        last_quote_number -> Nullable<Numeric>,
        order_seq_cust_meth_id -> Nullable<Varchar>,
        order_id_prefix -> Nullable<Varchar>,
        last_order_number -> Nullable<Numeric>,
        refund_payment_method_id -> Nullable<Varchar>,
        error_gl_journal_id -> Nullable<Varchar>,
        enable_accounting -> Nullable<Bpchar>,
        invoice_sequence_enum_id -> Nullable<Varchar>,
        order_sequence_enum_id -> Nullable<Varchar>,
        quote_sequence_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_attribute (party_id, attr_name) {
        party_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_benefit (role_type_id_from, role_type_id_to, party_id_from, party_id_to, benefit_type_id, from_date) {
        role_type_id_from -> Varchar,
        role_type_id_to -> Varchar,
        party_id_from -> Varchar,
        party_id_to -> Varchar,
        benefit_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        period_type_id -> Nullable<Varchar>,
        cost -> Nullable<Numeric>,
        actual_employer_paid_percent -> Nullable<Float8>,
        available_time -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_carrier_account (party_id, carrier_party_id, from_date) {
        party_id -> Varchar,
        carrier_party_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        account_number -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_classification (party_id, party_classification_group_id, from_date) {
        party_id -> Varchar,
        party_classification_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_classification_group (party_classification_group_id) {
        party_classification_group_id -> Varchar,
        party_classification_type_id -> Nullable<Varchar>,
        parent_group_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_classification_type (party_classification_type_id) {
        party_classification_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_contact_mech (party_id, contact_mech_id, from_date) {
        party_id -> Varchar,
        contact_mech_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        role_type_id -> Nullable<Varchar>,
        allow_solicitation -> Nullable<Bpchar>,
        extension -> Nullable<Varchar>,
        verified -> Nullable<Bpchar>,
        comments -> Nullable<Varchar>,
        years_with_contact_mech -> Nullable<Numeric>,
        months_with_contact_mech -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_contact_mech_purpose (party_id, contact_mech_id, contact_mech_purpose_type_id, from_date) {
        party_id -> Varchar,
        contact_mech_id -> Varchar,
        contact_mech_purpose_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_content (party_id, content_id, party_content_type_id, from_date) {
        party_id -> Varchar,
        content_id -> Varchar,
        party_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_content_type (party_content_type_id) {
        party_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_data_source (party_id, data_source_id, from_date) {
        party_id -> Varchar,
        data_source_id -> Varchar,
        from_date -> Timestamptz,
        visit_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        is_create -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_fixed_asset_assignment (party_id, role_type_id, fixed_asset_id, from_date) {
        party_id -> Varchar,
        role_type_id -> Varchar,
        fixed_asset_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        allocated_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_geo_point (party_id, geo_point_id, from_date) {
        party_id -> Varchar,
        geo_point_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_gl_account (organization_party_id, party_id, role_type_id, gl_account_type_id) {
        organization_party_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        gl_account_type_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_group (party_id) {
        party_id -> Varchar,
        group_name -> Nullable<Varchar>,
        group_name_local -> Nullable<Varchar>,
        office_site_name -> Nullable<Varchar>,
        annual_revenue -> Nullable<Numeric>,
        num_employees -> Nullable<Numeric>,
        ticker_symbol -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        logo_image_url -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_ics_avs_override (party_id) {
        party_id -> Varchar,
        avs_decline_string -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_identification (party_id, party_identification_type_id) {
        party_id -> Varchar,
        party_identification_type_id -> Varchar,
        id_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_identification_type (party_identification_type_id) {
        party_identification_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_invitation (party_invitation_id) {
        party_invitation_id -> Varchar,
        party_id_from -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        to_name -> Nullable<Varchar>,
        email_address -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        last_invite_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_invitation_group_assoc (party_invitation_id, party_id_to) {
        party_invitation_id -> Varchar,
        party_id_to -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_invitation_role_assoc (party_invitation_id, role_type_id) {
        party_invitation_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_name_history (party_id, change_date) {
        party_id -> Varchar,
        change_date -> Timestamptz,
        group_name -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        personal_title -> Nullable<Varchar>,
        suffix -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_need (party_need_id, party_id, role_type_id) {
        party_need_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        party_type_id -> Nullable<Varchar>,
        need_type_id -> Nullable<Varchar>,
        communication_event_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        product_category_id -> Nullable<Varchar>,
        visit_id -> Nullable<Varchar>,
        datetime_recorded -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_note (party_id, note_id) {
        party_id -> Varchar,
        note_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_pref_doc_type_tpl (party_pref_doc_type_tpl_id) {
        party_pref_doc_type_tpl_id -> Varchar,
        party_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        invoice_type_id -> Nullable<Varchar>,
        order_type_id -> Nullable<Varchar>,
        quote_type_id -> Nullable<Varchar>,
        custom_screen_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_profile_default (party_id, product_store_id) {
        party_id -> Varchar,
        product_store_id -> Varchar,
        default_ship_addr -> Nullable<Varchar>,
        default_bill_addr -> Nullable<Varchar>,
        default_pay_meth -> Nullable<Varchar>,
        default_ship_meth -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_qual (party_id, party_qual_type_id, from_date) {
        party_id -> Varchar,
        party_qual_type_id -> Varchar,
        qualification_desc -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        verif_status_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_qual_type (party_qual_type_id) {
        party_qual_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_rate_new (party_id, rate_type_id, from_date) {
        party_id -> Varchar,
        rate_type_id -> Varchar,
        default_rate -> Nullable<Bpchar>,
        percentage_used -> Nullable<Float8>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_relationship (party_id_from, party_id_to, role_type_id_from, role_type_id_to, from_date) {
        party_id_from -> Varchar,
        party_id_to -> Varchar,
        role_type_id_from -> Varchar,
        role_type_id_to -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Varchar>,
        relationship_name -> Nullable<Varchar>,
        security_group_id -> Nullable<Varchar>,
        priority_type_id -> Nullable<Varchar>,
        party_relationship_type_id -> Nullable<Varchar>,
        permissions_enum_id -> Nullable<Varchar>,
        position_title -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_relationship_type (party_relationship_type_id) {
        party_relationship_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        party_relationship_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        role_type_id_valid_from -> Nullable<Varchar>,
        role_type_id_valid_to -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_resume (resume_id) {
        resume_id -> Varchar,
        party_id -> Nullable<Varchar>,
        content_id -> Nullable<Varchar>,
        resume_date -> Nullable<Timestamptz>,
        resume_text -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_role (party_id, role_type_id) {
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_skill (party_id, skill_type_id) {
        party_id -> Varchar,
        skill_type_id -> Varchar,
        years_experience -> Nullable<Numeric>,
        rating -> Nullable<Numeric>,
        skill_level -> Nullable<Numeric>,
        started_using_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_status (status_id, party_id, status_date) {
        status_id -> Varchar,
        party_id -> Varchar,
        status_date -> Timestamptz,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_tax_auth_info (party_id, tax_auth_geo_id, tax_auth_party_id, from_date) {
        party_id -> Varchar,
        tax_auth_geo_id -> Varchar,
        tax_auth_party_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        party_tax_id -> Nullable<Varchar>,
        is_exempt -> Nullable<Bpchar>,
        is_nexus -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_type (party_type_id) {
        party_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    party_type_attr (party_type_id, attr_name) {
        party_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    pay_grade (pay_grade_id) {
        pay_grade_id -> Varchar,
        pay_grade_name -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    pay_history (role_type_id_from, role_type_id_to, party_id_from, party_id_to, empl_from_date, from_date) {
        role_type_id_from -> Varchar,
        role_type_id_to -> Varchar,
        party_id_from -> Varchar,
        party_id_to -> Varchar,
        empl_from_date -> Timestamptz,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        salary_step_seq_id -> Nullable<Varchar>,
        pay_grade_id -> Nullable<Varchar>,
        period_type_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    pay_pal_payment_method (payment_method_id) {
        payment_method_id -> Varchar,
        payer_id -> Nullable<Varchar>,
        express_checkout_token -> Nullable<Varchar>,
        payer_status -> Nullable<Varchar>,
        avs_addr -> Nullable<Bpchar>,
        avs_zip -> Nullable<Bpchar>,
        correlation_id -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        transaction_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment (payment_id) {
        payment_id -> Varchar,
        payment_type_id -> Nullable<Varchar>,
        payment_method_type_id -> Nullable<Varchar>,
        payment_method_id -> Nullable<Varchar>,
        payment_gateway_response_id -> Nullable<Varchar>,
        payment_preference_id -> Nullable<Varchar>,
        party_id_from -> Nullable<Varchar>,
        party_id_to -> Nullable<Varchar>,
        role_type_id_to -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        effective_date -> Nullable<Timestamptz>,
        payment_ref_num -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        fin_account_trans_id -> Nullable<Varchar>,
        override_gl_account_id -> Nullable<Varchar>,
        actual_currency_amount -> Nullable<Numeric>,
        actual_currency_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_application (payment_application_id) {
        payment_application_id -> Varchar,
        payment_id -> Nullable<Varchar>,
        invoice_id -> Nullable<Varchar>,
        invoice_item_seq_id -> Nullable<Varchar>,
        billing_account_id -> Nullable<Varchar>,
        override_gl_account_id -> Nullable<Varchar>,
        to_payment_id -> Nullable<Varchar>,
        tax_auth_geo_id -> Nullable<Varchar>,
        amount_applied -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_attribute (payment_id, attr_name) {
        payment_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_budget_allocation (budget_id, budget_item_seq_id, payment_id) {
        budget_id -> Varchar,
        budget_item_seq_id -> Varchar,
        payment_id -> Varchar,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_content (content_id, payment_id, payment_content_type_id, from_date) {
        payment_id -> Varchar,
        payment_content_type_id -> Varchar,
        content_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_content_type (payment_content_type_id) {
        payment_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_authorize_net (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        transaction_url -> Nullable<Varchar>,
        certificate_alias -> Nullable<Varchar>,
        api_version -> Nullable<Varchar>,
        delimited_data -> Nullable<Varchar>,
        delimiter_char -> Nullable<Varchar>,
        cp_version -> Nullable<Varchar>,
        cp_market_type -> Nullable<Varchar>,
        cp_device_type -> Nullable<Varchar>,
        method -> Nullable<Varchar>,
        email_customer -> Nullable<Varchar>,
        email_merchant -> Nullable<Varchar>,
        test_mode -> Nullable<Varchar>,
        relay_response -> Nullable<Varchar>,
        tran_key -> Nullable<Varchar>,
        user_id -> Nullable<Varchar>,
        pwd -> Nullable<Varchar>,
        trans_description -> Nullable<Varchar>,
        duplicate_window -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_clear_commerce (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        source_id -> Nullable<Varchar>,
        group_id -> Nullable<Varchar>,
        client_id -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        pwd -> Nullable<Varchar>,
        user_alias -> Nullable<Varchar>,
        effective_alias -> Nullable<Varchar>,
        process_mode -> Nullable<Bpchar>,
        server_u_r_l -> Nullable<Varchar>,
        enable_c_v_m -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_config (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        payment_gateway_config_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_config_type (payment_gateway_config_type_id) {
        payment_gateway_config_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_cyber_source (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        merchant_id -> Nullable<Varchar>,
        api_version -> Nullable<Varchar>,
        production -> Nullable<Varchar>,
        keys_dir -> Nullable<Varchar>,
        keys_file -> Nullable<Varchar>,
        log_enabled -> Nullable<Varchar>,
        log_dir -> Nullable<Varchar>,
        log_file -> Nullable<Varchar>,
        log_size -> Nullable<Numeric>,
        merchant_descr -> Nullable<Varchar>,
        merchant_contact -> Nullable<Varchar>,
        auto_bill -> Nullable<Varchar>,
        enable_dav -> Nullable<Bpchar>,
        fraud_score -> Nullable<Bpchar>,
        ignore_avs -> Nullable<Varchar>,
        disable_bill_avs -> Nullable<Bpchar>,
        avs_decline_codes -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_eway (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        customer_id -> Nullable<Varchar>,
        refund_pwd -> Nullable<Varchar>,
        test_mode -> Nullable<Varchar>,
        enable_cvn -> Nullable<Varchar>,
        enable_beagle -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_first_data (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        app_name -> Nullable<Varchar>,
        api_key -> Nullable<Varchar>,
        api_secret -> Nullable<Varchar>,
        transaction_url -> Nullable<Varchar>,
        enable_data_vault -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_orbital (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        username -> Nullable<Varchar>,
        connection_password -> Nullable<Varchar>,
        merchant_id -> Nullable<Varchar>,
        engine_class -> Nullable<Varchar>,
        host_name -> Nullable<Varchar>,
        port -> Nullable<Numeric>,
        host_name_failover -> Nullable<Varchar>,
        port_failover -> Nullable<Numeric>,
        connection_timeout_seconds -> Nullable<Numeric>,
        read_timeout_seconds -> Nullable<Numeric>,
        authorization_u_r_i -> Nullable<Varchar>,
        sdk_version -> Nullable<Varchar>,
        ssl_socket_factory -> Nullable<Varchar>,
        response_type -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_pay_pal (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        business_email -> Nullable<Varchar>,
        api_user_name -> Nullable<Varchar>,
        api_password -> Nullable<Varchar>,
        api_signature -> Nullable<Varchar>,
        api_environment -> Nullable<Varchar>,
        notify_url -> Nullable<Varchar>,
        return_url -> Nullable<Varchar>,
        cancel_return_url -> Nullable<Varchar>,
        image_url -> Nullable<Varchar>,
        confirm_template -> Nullable<Varchar>,
        redirect_url -> Nullable<Varchar>,
        confirm_url -> Nullable<Varchar>,
        shipping_callback_url -> Nullable<Varchar>,
        require_confirmed_shipping -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_payflow_pro (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        certs_path -> Nullable<Varchar>,
        host_address -> Nullable<Varchar>,
        host_port -> Nullable<Numeric>,
        timeout -> Nullable<Numeric>,
        proxy_address -> Nullable<Varchar>,
        proxy_port -> Nullable<Numeric>,
        proxy_logon -> Nullable<Varchar>,
        proxy_password -> Nullable<Varchar>,
        vendor -> Nullable<Varchar>,
        user_id -> Nullable<Varchar>,
        pwd -> Nullable<Varchar>,
        partner -> Nullable<Varchar>,
        check_avs -> Nullable<Bpchar>,
        check_cvv2 -> Nullable<Bpchar>,
        pre_auth -> Nullable<Bpchar>,
        enable_transmit -> Nullable<Varchar>,
        log_file_name -> Nullable<Varchar>,
        logging_level -> Nullable<Numeric>,
        max_log_file_size -> Nullable<Numeric>,
        stack_trace_on -> Nullable<Bpchar>,
        redirect_url -> Nullable<Varchar>,
        return_url -> Nullable<Varchar>,
        cancel_return_url -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_resp_msg (payment_gateway_resp_msg_id) {
        payment_gateway_resp_msg_id -> Varchar,
        payment_gateway_response_id -> Nullable<Varchar>,
        pgr_message -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_response (payment_gateway_response_id) {
        payment_gateway_response_id -> Varchar,
        payment_service_type_enum_id -> Nullable<Varchar>,
        order_payment_preference_id -> Nullable<Varchar>,
        payment_method_type_id -> Nullable<Varchar>,
        payment_method_id -> Nullable<Varchar>,
        trans_code_enum_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        reference_num -> Nullable<Varchar>,
        alt_reference -> Nullable<Varchar>,
        sub_reference -> Nullable<Varchar>,
        gateway_code -> Nullable<Varchar>,
        gateway_flag -> Nullable<Varchar>,
        gateway_avs_result -> Nullable<Varchar>,
        gateway_cv_result -> Nullable<Varchar>,
        gateway_score_result -> Nullable<Varchar>,
        gateway_message -> Nullable<Varchar>,
        transaction_date -> Nullable<Timestamptz>,
        result_declined -> Nullable<Bpchar>,
        result_nsf -> Nullable<Bpchar>,
        result_bad_expire -> Nullable<Bpchar>,
        result_bad_card_number -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_sage_pay (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        vendor -> Nullable<Varchar>,
        production_host -> Nullable<Varchar>,
        testing_host -> Nullable<Varchar>,
        sage_pay_mode -> Nullable<Varchar>,
        protocol_version -> Nullable<Varchar>,
        authentication_trans_type -> Nullable<Varchar>,
        authentication_url -> Nullable<Varchar>,
        authorise_trans_type -> Nullable<Varchar>,
        authorise_url -> Nullable<Varchar>,
        release_trans_type -> Nullable<Varchar>,
        release_url -> Nullable<Varchar>,
        void_url -> Nullable<Varchar>,
        refund_url -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_secure_pay (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        merchant_id -> Nullable<Varchar>,
        pwd -> Nullable<Varchar>,
        server_u_r_l -> Nullable<Varchar>,
        process_timeout -> Nullable<Numeric>,
        enable_amount_round -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gateway_world_pay (payment_gateway_config_id) {
        payment_gateway_config_id -> Varchar,
        redirect_url -> Nullable<Varchar>,
        inst_id -> Nullable<Varchar>,
        auth_mode -> Nullable<Bpchar>,
        fix_contact -> Nullable<Bpchar>,
        hide_contact -> Nullable<Bpchar>,
        hide_currency -> Nullable<Bpchar>,
        lang_id -> Nullable<Varchar>,
        no_language_menu -> Nullable<Bpchar>,
        with_delivery -> Nullable<Bpchar>,
        test_mode -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_gl_account_type_map (payment_type_id, organization_party_id) {
        payment_type_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_group (payment_group_id) {
        payment_group_id -> Varchar,
        payment_group_type_id -> Nullable<Varchar>,
        payment_group_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_group_member (payment_group_id, payment_id, from_date) {
        payment_group_id -> Varchar,
        payment_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_group_type (payment_group_type_id) {
        payment_group_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_method (payment_method_id) {
        payment_method_id -> Varchar,
        payment_method_type_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        gl_account_id -> Nullable<Varchar>,
        fin_account_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_method_type (payment_method_type_id) {
        payment_method_type_id -> Varchar,
        description -> Nullable<Varchar>,
        default_gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_method_type_gl_account (payment_method_type_id, organization_party_id) {
        payment_method_type_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_type (payment_type_id) {
        payment_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payment_type_attr (payment_type_id, attr_name) {
        payment_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    payroll_preference (party_id, role_type_id, payroll_preference_seq_id) {
        party_id -> Varchar,
        role_type_id -> Varchar,
        payroll_preference_seq_id -> Varchar,
        deduction_type_id -> Nullable<Varchar>,
        payment_method_type_id -> Nullable<Varchar>,
        period_type_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        percentage -> Nullable<Float8>,
        flat_amount -> Nullable<Numeric>,
        routing_number -> Nullable<Varchar>,
        account_number -> Nullable<Varchar>,
        bank_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    perf_rating_type (perf_rating_type_id) {
        perf_rating_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    perf_review (employee_party_id, employee_role_type_id, perf_review_id) {
        employee_party_id -> Varchar,
        employee_role_type_id -> Varchar,
        perf_review_id -> Varchar,
        manager_party_id -> Nullable<Varchar>,
        manager_role_type_id -> Nullable<Varchar>,
        payment_id -> Nullable<Varchar>,
        empl_position_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    perf_review_item (employee_party_id, employee_role_type_id, perf_review_id, perf_review_item_seq_id) {
        employee_party_id -> Varchar,
        employee_role_type_id -> Varchar,
        perf_review_id -> Varchar,
        perf_review_item_seq_id -> Varchar,
        perf_review_item_type_id -> Nullable<Varchar>,
        perf_rating_type_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    perf_review_item_type (perf_review_item_type_id) {
        perf_review_item_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    performance_note (party_id, role_type_id, from_date) {
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        communication_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    period_type (period_type_id) {
        period_type_id -> Varchar,
        description -> Nullable<Varchar>,
        period_length -> Nullable<Numeric>,
        uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    person (party_id) {
        party_id -> Varchar,
        salutation -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        middle_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        personal_title -> Nullable<Varchar>,
        suffix -> Nullable<Varchar>,
        nickname -> Nullable<Varchar>,
        first_name_local -> Nullable<Varchar>,
        middle_name_local -> Nullable<Varchar>,
        last_name_local -> Nullable<Varchar>,
        other_local -> Nullable<Varchar>,
        member_id -> Nullable<Varchar>,
        gender -> Nullable<Bpchar>,
        birth_date -> Nullable<Date>,
        deceased_date -> Nullable<Date>,
        height -> Nullable<Float8>,
        weight -> Nullable<Float8>,
        mothers_maiden_name -> Nullable<Varchar>,
        marital_status -> Nullable<Bpchar>,
        marital_status_enum_id -> Nullable<Varchar>,
        social_security_number -> Nullable<Varchar>,
        passport_number -> Nullable<Varchar>,
        passport_expire_date -> Nullable<Date>,
        total_years_work_experience -> Nullable<Float8>,
        comments -> Nullable<Varchar>,
        employment_status_enum_id -> Nullable<Varchar>,
        residence_status_enum_id -> Nullable<Varchar>,
        occupation -> Nullable<Varchar>,
        years_with_employer -> Nullable<Numeric>,
        months_with_employer -> Nullable<Numeric>,
        existing_customer -> Nullable<Bpchar>,
        card_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    person_training (party_id, training_class_type_id, from_date) {
        party_id -> Varchar,
        training_request_id -> Nullable<Varchar>,
        training_class_type_id -> Varchar,
        work_effort_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        approver_id -> Nullable<Varchar>,
        approval_status -> Nullable<Varchar>,
        reason -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    physical_inventory (physical_inventory_id) {
        physical_inventory_id -> Varchar,
        physical_inventory_date -> Nullable<Timestamptz>,
        party_id -> Nullable<Varchar>,
        general_comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    picklist (picklist_id) {
        picklist_id -> Varchar,
        description -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        shipment_method_type_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        picklist_date -> Nullable<Timestamptz>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    picklist_bin (picklist_bin_id) {
        picklist_bin_id -> Varchar,
        picklist_id -> Nullable<Varchar>,
        bin_location_number -> Nullable<Numeric>,
        primary_order_id -> Nullable<Varchar>,
        primary_ship_group_seq_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    picklist_item (picklist_bin_id, order_id, order_item_seq_id, ship_group_seq_id, inventory_item_id) {
        picklist_bin_id -> Varchar,
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        ship_group_seq_id -> Varchar,
        inventory_item_id -> Varchar,
        item_status_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    picklist_role (picklist_id, party_id, role_type_id, from_date) {
        picklist_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    picklist_status (picklist_id, status_date) {
        picklist_id -> Varchar,
        status_date -> Timestamptz,
        change_by_user_login_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        status_id_to -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    picklist_status_history (picklist_id, change_date) {
        picklist_id -> Varchar,
        change_date -> Timestamptz,
        change_user_login_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        status_id_to -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    platform_type (platform_type_id) {
        platform_type_id -> Varchar,
        platform_name -> Nullable<Varchar>,
        platform_version -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    portal_page (portal_page_id) {
        portal_page_id -> Varchar,
        portal_page_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        owner_user_login_id -> Nullable<Varchar>,
        original_portal_page_id -> Nullable<Varchar>,
        parent_portal_page_id -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        security_group_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        help_content_id -> Nullable<Varchar>,
    }
}

table! {
    portal_page_column (portal_page_id, column_seq_id) {
        portal_page_id -> Varchar,
        column_seq_id -> Varchar,
        column_width_pixels -> Nullable<Numeric>,
        column_width_percentage -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    portal_page_portlet (portal_page_id, portal_portlet_id, portlet_seq_id) {
        portal_page_id -> Varchar,
        portal_portlet_id -> Varchar,
        portlet_seq_id -> Varchar,
        column_seq_id -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    portal_portlet (portal_portlet_id) {
        portal_portlet_id -> Varchar,
        portlet_name -> Nullable<Varchar>,
        screen_name -> Nullable<Varchar>,
        screen_location -> Nullable<Varchar>,
        edit_form_name -> Nullable<Varchar>,
        edit_form_location -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        screenshot -> Nullable<Varchar>,
        security_service_name -> Nullable<Varchar>,
        security_main_action -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    portlet_attribute (portal_page_id, portal_portlet_id, portlet_seq_id, attr_name) {
        portal_page_id -> Varchar,
        portal_portlet_id -> Varchar,
        portlet_seq_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        attr_type -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    portlet_category (portlet_category_id) {
        portlet_category_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    portlet_portlet_category (portal_portlet_id, portlet_category_id) {
        portal_portlet_id -> Varchar,
        portlet_category_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    pos_terminal (pos_terminal_id) {
        pos_terminal_id -> Varchar,
        facility_id -> Nullable<Varchar>,
        push_entity_sync_id -> Nullable<Varchar>,
        terminal_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    pos_terminal_intern_tx (pos_terminal_log_id) {
        pos_terminal_log_id -> Varchar,
        paid_amount -> Nullable<Numeric>,
        reason_comment -> Nullable<Varchar>,
        reason_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    pos_terminal_log (pos_terminal_log_id) {
        pos_terminal_log_id -> Varchar,
        pos_terminal_id -> Nullable<Varchar>,
        transaction_id -> Nullable<Varchar>,
        item_count -> Nullable<Numeric>,
        order_id -> Nullable<Varchar>,
        return_id -> Nullable<Varchar>,
        user_login_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        log_start_date_time -> Nullable<Timestamptz>,
        log_end_date_time -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    pos_terminal_state (pos_terminal_id, opened_date) {
        pos_terminal_id -> Varchar,
        opened_date -> Timestamptz,
        closed_date -> Nullable<Timestamptz>,
        starting_tx_id -> Nullable<Varchar>,
        ending_tx_id -> Nullable<Varchar>,
        opened_by_user_login_id -> Nullable<Varchar>,
        closed_by_user_login_id -> Nullable<Varchar>,
        starting_drawer_amount -> Nullable<Numeric>,
        actual_ending_cash -> Nullable<Numeric>,
        actual_ending_check -> Nullable<Numeric>,
        actual_ending_cc -> Nullable<Numeric>,
        actual_ending_gc -> Nullable<Numeric>,
        actual_ending_other -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    postal_address (contact_mech_id) {
        contact_mech_id -> Varchar,
        to_name -> Nullable<Varchar>,
        attn_name -> Nullable<Varchar>,
        address1 -> Nullable<Varchar>,
        address2 -> Nullable<Varchar>,
        house_number -> Nullable<Numeric>,
        house_number_ext -> Nullable<Varchar>,
        directions -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        city_geo_id -> Nullable<Varchar>,
        postal_code -> Nullable<Varchar>,
        postal_code_ext -> Nullable<Varchar>,
        country_geo_id -> Nullable<Varchar>,
        state_province_geo_id -> Nullable<Varchar>,
        county_geo_id -> Nullable<Varchar>,
        municipality_geo_id -> Nullable<Varchar>,
        postal_code_geo_id -> Nullable<Varchar>,
        geo_point_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    postal_address_boundary (contact_mech_id, geo_id) {
        contact_mech_id -> Varchar,
        geo_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    priority_type (priority_type_id) {
        priority_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_catalog (prod_catalog_id) {
        prod_catalog_id -> Varchar,
        catalog_name -> Nullable<Varchar>,
        use_quick_add -> Nullable<Bpchar>,
        style_sheet -> Nullable<Varchar>,
        header_logo -> Nullable<Varchar>,
        content_path_prefix -> Nullable<Varchar>,
        template_path_prefix -> Nullable<Varchar>,
        view_allow_perm_reqd -> Nullable<Bpchar>,
        purchase_allow_perm_reqd -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_catalog_category (prod_catalog_id, product_category_id, prod_catalog_category_type_id, from_date) {
        prod_catalog_id -> Varchar,
        product_category_id -> Varchar,
        prod_catalog_category_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_catalog_category_type (prod_catalog_category_type_id) {
        prod_catalog_category_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_catalog_inv_facility (prod_catalog_id, facility_id, from_date) {
        prod_catalog_id -> Varchar,
        facility_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_catalog_role (party_id, role_type_id, prod_catalog_id, from_date) {
        party_id -> Varchar,
        role_type_id -> Varchar,
        prod_catalog_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_conf_item_content (config_item_id, content_id, conf_item_content_type_id, from_date) {
        config_item_id -> Varchar,
        content_id -> Varchar,
        conf_item_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_conf_item_content_type (conf_item_content_type_id) {
        conf_item_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    prod_promo_code_contact_mech (product_promo_code_id, contact_mech_id) {
        product_promo_code_id -> Varchar,
        contact_mech_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product (product_id) {
        product_id -> Varchar,
        product_type_id -> Nullable<Varchar>,
        primary_product_category_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        introduction_date -> Nullable<Timestamptz>,
        release_date -> Nullable<Timestamptz>,
        support_discontinuation_date -> Nullable<Timestamptz>,
        sales_discontinuation_date -> Nullable<Timestamptz>,
        sales_disc_when_not_avail -> Nullable<Bpchar>,
        internal_name -> Nullable<Varchar>,
        brand_name -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        product_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        long_description -> Nullable<Text>,
        price_detail_text -> Nullable<Varchar>,
        small_image_url -> Nullable<Varchar>,
        medium_image_url -> Nullable<Varchar>,
        large_image_url -> Nullable<Varchar>,
        detail_image_url -> Nullable<Varchar>,
        original_image_url -> Nullable<Varchar>,
        detail_screen -> Nullable<Varchar>,
        inventory_message -> Nullable<Varchar>,
        inventory_item_type_id -> Nullable<Varchar>,
        require_inventory -> Nullable<Bpchar>,
        quantity_uom_id -> Nullable<Varchar>,
        quantity_included -> Nullable<Numeric>,
        pieces_included -> Nullable<Numeric>,
        require_amount -> Nullable<Bpchar>,
        fixed_amount -> Nullable<Numeric>,
        amount_uom_type_id -> Nullable<Varchar>,
        weight_uom_id -> Nullable<Varchar>,
        shipping_weight -> Nullable<Numeric>,
        product_weight -> Nullable<Numeric>,
        height_uom_id -> Nullable<Varchar>,
        product_height -> Nullable<Numeric>,
        shipping_height -> Nullable<Numeric>,
        width_uom_id -> Nullable<Varchar>,
        product_width -> Nullable<Numeric>,
        shipping_width -> Nullable<Numeric>,
        depth_uom_id -> Nullable<Varchar>,
        product_depth -> Nullable<Numeric>,
        shipping_depth -> Nullable<Numeric>,
        diameter_uom_id -> Nullable<Varchar>,
        product_diameter -> Nullable<Numeric>,
        product_rating -> Nullable<Numeric>,
        rating_type_enum -> Nullable<Varchar>,
        returnable -> Nullable<Bpchar>,
        taxable -> Nullable<Bpchar>,
        charge_shipping -> Nullable<Bpchar>,
        auto_create_keywords -> Nullable<Bpchar>,
        include_in_promotions -> Nullable<Bpchar>,
        is_virtual -> Nullable<Bpchar>,
        is_variant -> Nullable<Bpchar>,
        virtual_variant_method_enum -> Nullable<Varchar>,
        origin_geo_id -> Nullable<Varchar>,
        requirement_method_enum_id -> Nullable<Varchar>,
        bill_of_material_level -> Nullable<Numeric>,
        reserv_max_persons -> Nullable<Numeric>,
        reserv2nd_p_p_perc -> Nullable<Numeric>,
        reserv_nth_p_p_perc -> Nullable<Numeric>,
        config_id -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        in_shipping_box -> Nullable<Bpchar>,
        default_shipment_box_type_id -> Nullable<Varchar>,
        lot_id_filled_in -> Nullable<Varchar>,
        order_decimal_quantity -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_assoc (product_id, product_id_to, product_assoc_type_id, from_date) {
        product_id -> Varchar,
        product_id_to -> Varchar,
        product_assoc_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        reason -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        scrap_factor -> Nullable<Numeric>,
        instruction -> Nullable<Varchar>,
        routing_work_effort_id -> Nullable<Varchar>,
        estimate_calc_method -> Nullable<Varchar>,
        recurrence_info_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_assoc_type (product_assoc_type_id) {
        product_assoc_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_attribute (product_id, attr_name) {
        product_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_type -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_average_cost (product_average_cost_type_id, organization_party_id, product_id, facility_id, from_date) {
        product_average_cost_type_id -> Varchar,
        organization_party_id -> Varchar,
        product_id -> Varchar,
        facility_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        average_cost -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_average_cost_type (product_average_cost_type_id) {
        product_average_cost_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_calculated_info (product_id) {
        product_id -> Varchar,
        total_quantity_ordered -> Nullable<Numeric>,
        total_times_viewed -> Nullable<Numeric>,
        average_customer_rating -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category (product_category_id) {
        product_category_id -> Varchar,
        product_category_type_id -> Nullable<Varchar>,
        primary_parent_category_id -> Nullable<Varchar>,
        category_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        long_description -> Nullable<Text>,
        category_image_url -> Nullable<Varchar>,
        link_one_image_url -> Nullable<Varchar>,
        link_two_image_url -> Nullable<Varchar>,
        detail_screen -> Nullable<Varchar>,
        show_in_select -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_attribute (product_category_id, attr_name) {
        product_category_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_content (product_category_id, content_id, prod_cat_content_type_id, from_date) {
        product_category_id -> Varchar,
        content_id -> Varchar,
        prod_cat_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        purchase_from_date -> Nullable<Timestamptz>,
        purchase_thru_date -> Nullable<Timestamptz>,
        use_count_limit -> Nullable<Numeric>,
        use_days_limit -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_content_type (prod_cat_content_type_id) {
        prod_cat_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_gl_account (product_category_id, organization_party_id, gl_account_type_id) {
        product_category_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_type_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_link (product_category_id, link_seq_id, from_date) {
        product_category_id -> Varchar,
        link_seq_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        title_text -> Nullable<Varchar>,
        detail_text -> Nullable<Text>,
        image_url -> Nullable<Varchar>,
        image_two_url -> Nullable<Varchar>,
        link_type_enum_id -> Nullable<Varchar>,
        link_info -> Nullable<Varchar>,
        detail_sub_screen -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_member (product_category_id, product_id, from_date) {
        product_category_id -> Varchar,
        product_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_role (product_category_id, party_id, role_type_id, from_date) {
        product_category_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_rollup (product_category_id, parent_product_category_id, from_date) {
        product_category_id -> Varchar,
        parent_product_category_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_type (product_category_type_id) {
        product_category_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_category_type_attr (product_category_type_id, attr_name) {
        product_category_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_config (product_id, config_item_id, sequence_num, from_date) {
        product_id -> Varchar,
        config_item_id -> Varchar,
        sequence_num -> Numeric,
        from_date -> Timestamptz,
        description -> Nullable<Varchar>,
        long_description -> Nullable<Text>,
        config_type_id -> Nullable<Varchar>,
        default_config_option_id -> Nullable<Varchar>,
        thru_date -> Nullable<Timestamptz>,
        is_mandatory -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_config_config (config_id, config_item_id, config_option_id, sequence_num) {
        config_id -> Varchar,
        config_item_id -> Varchar,
        sequence_num -> Numeric,
        config_option_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_config_item (config_item_id) {
        config_item_id -> Varchar,
        config_item_type_id -> Nullable<Varchar>,
        config_item_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        long_description -> Nullable<Text>,
        image_url -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_config_option (config_item_id, config_option_id) {
        config_item_id -> Varchar,
        config_option_id -> Varchar,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        config_option_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_config_option_iactn (config_item_id, config_option_id, config_item_id_to, config_option_id_to, sequence_num) {
        config_item_id -> Varchar,
        config_option_id -> Varchar,
        config_item_id_to -> Varchar,
        config_option_id_to -> Varchar,
        sequence_num -> Numeric,
        config_iactn_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_config_product (config_item_id, config_option_id, product_id) {
        config_item_id -> Varchar,
        config_option_id -> Varchar,
        product_id -> Varchar,
        quantity -> Nullable<Numeric>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_config_stats (config_id, product_id) {
        config_id -> Varchar,
        product_id -> Varchar,
        num_of_confs -> Nullable<Numeric>,
        config_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_content (product_id, content_id, product_content_type_id, from_date) {
        product_id -> Varchar,
        content_id -> Varchar,
        product_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        purchase_from_date -> Nullable<Timestamptz>,
        purchase_thru_date -> Nullable<Timestamptz>,
        use_count_limit -> Nullable<Numeric>,
        use_time -> Nullable<Numeric>,
        use_time_uom_id -> Nullable<Varchar>,
        use_role_type_id -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_content_type (product_content_type_id) {
        product_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_cost_component_calc (product_id, cost_component_type_id, from_date) {
        product_id -> Varchar,
        cost_component_type_id -> Varchar,
        cost_component_calc_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        sequence_num -> Nullable<Numeric>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_facility (product_id, facility_id) {
        product_id -> Varchar,
        facility_id -> Varchar,
        minimum_stock -> Nullable<Numeric>,
        reorder_quantity -> Nullable<Numeric>,
        days_to_ship -> Nullable<Numeric>,
        replenish_method_enum_id -> Nullable<Varchar>,
        last_inventory_count -> Nullable<Numeric>,
        requirement_method_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_facility_assoc (product_id, facility_id, facility_id_to, facility_assoc_type_id, from_date) {
        product_id -> Varchar,
        facility_id -> Varchar,
        facility_id_to -> Varchar,
        facility_assoc_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        transit_time -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_facility_location (product_id, facility_id, location_seq_id) {
        product_id -> Varchar,
        facility_id -> Varchar,
        location_seq_id -> Varchar,
        minimum_stock -> Nullable<Numeric>,
        move_quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature (product_feature_id) {
        product_feature_id -> Varchar,
        product_feature_type_id -> Nullable<Varchar>,
        product_feature_category_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        uom_id -> Nullable<Varchar>,
        number_specified -> Nullable<Numeric>,
        default_amount -> Nullable<Numeric>,
        default_sequence_num -> Nullable<Numeric>,
        abbrev -> Nullable<Varchar>,
        id_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_appl (product_id, product_feature_id, from_date) {
        product_id -> Varchar,
        product_feature_id -> Varchar,
        product_feature_appl_type_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        amount -> Nullable<Numeric>,
        recurring_amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_appl_attr (product_id, product_feature_id, from_date, attr_name) {
        product_id -> Varchar,
        product_feature_id -> Varchar,
        from_date -> Timestamptz,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_appl_type (product_feature_appl_type_id) {
        product_feature_appl_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_cat_grp_appl (product_category_id, product_feature_group_id, from_date) {
        product_category_id -> Varchar,
        product_feature_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_category (product_feature_category_id) {
        product_feature_category_id -> Varchar,
        parent_category_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_category_appl (product_category_id, product_feature_category_id, from_date) {
        product_category_id -> Varchar,
        product_feature_category_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_data_resource (data_resource_id, product_feature_id) {
        data_resource_id -> Varchar,
        product_feature_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_group (product_feature_group_id) {
        product_feature_group_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_group_appl (product_feature_group_id, product_feature_id, from_date) {
        product_feature_group_id -> Varchar,
        product_feature_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_iactn (product_feature_id, product_feature_id_to) {
        product_feature_id -> Varchar,
        product_feature_id_to -> Varchar,
        product_feature_iactn_type_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_iactn_type (product_feature_iactn_type_id) {
        product_feature_iactn_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_price (product_feature_id, product_price_type_id, currency_uom_id, from_date) {
        product_feature_id -> Varchar,
        product_price_type_id -> Varchar,
        currency_uom_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        price -> Nullable<Numeric>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_feature_type (product_feature_type_id) {
        product_feature_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_geo (product_id, geo_id) {
        product_id -> Varchar,
        geo_id -> Varchar,
        product_geo_enum_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_gl_account (product_id, organization_party_id, gl_account_type_id) {
        product_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_type_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_group_order (group_order_id) {
        group_order_id -> Varchar,
        product_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Varchar>,
        req_order_qty -> Nullable<Numeric>,
        sold_order_qty -> Nullable<Numeric>,
        job_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_keyword_new (product_id, keyword, keyword_type_id) {
        product_id -> Varchar,
        keyword -> Varchar,
        keyword_type_id -> Varchar,
        relevancy_weight -> Nullable<Numeric>,
        status_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_maint (product_id, product_maint_seq_id) {
        product_id -> Varchar,
        product_maint_seq_id -> Varchar,
        product_maint_type_id -> Nullable<Varchar>,
        maint_name -> Nullable<Varchar>,
        maint_template_work_effort_id -> Nullable<Varchar>,
        interval_quantity -> Nullable<Numeric>,
        interval_uom_id -> Nullable<Varchar>,
        interval_meter_type_id -> Nullable<Varchar>,
        repeat_count -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_maint_type (product_maint_type_id) {
        product_maint_type_id -> Varchar,
        description -> Nullable<Varchar>,
        parent_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_manufacturing_rule (rule_id) {
        rule_id -> Varchar,
        product_id -> Nullable<Varchar>,
        product_id_for -> Nullable<Varchar>,
        product_id_in -> Nullable<Varchar>,
        rule_seq_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        product_id_in_subst -> Nullable<Varchar>,
        product_feature -> Nullable<Varchar>,
        rule_operator -> Nullable<Varchar>,
        quantity -> Nullable<Float8>,
        description -> Nullable<Varchar>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_meter (product_id, product_meter_type_id) {
        product_id -> Varchar,
        product_meter_type_id -> Varchar,
        meter_uom_id -> Nullable<Varchar>,
        meter_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_meter_type (product_meter_type_id) {
        product_meter_type_id -> Varchar,
        description -> Nullable<Varchar>,
        default_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_order_item (order_id, order_item_seq_id, engagement_id, engagement_item_seq_id) {
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        engagement_id -> Varchar,
        engagement_item_seq_id -> Varchar,
        product_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_payment_method_type (product_id, payment_method_type_id, product_price_purpose_id, from_date) {
        product_id -> Varchar,
        payment_method_type_id -> Varchar,
        product_price_purpose_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price (product_id, product_price_type_id, product_price_purpose_id, currency_uom_id, product_store_group_id, from_date) {
        product_id -> Varchar,
        product_price_type_id -> Varchar,
        product_price_purpose_id -> Varchar,
        currency_uom_id -> Varchar,
        product_store_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        price -> Nullable<Numeric>,
        term_uom_id -> Nullable<Varchar>,
        custom_price_calc_service -> Nullable<Varchar>,
        price_without_tax -> Nullable<Numeric>,
        price_with_tax -> Nullable<Numeric>,
        tax_amount -> Nullable<Numeric>,
        tax_percentage -> Nullable<Numeric>,
        tax_auth_party_id -> Nullable<Varchar>,
        tax_auth_geo_id -> Nullable<Varchar>,
        tax_in_price -> Nullable<Bpchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_action (product_price_rule_id, product_price_action_seq_id) {
        product_price_rule_id -> Varchar,
        product_price_action_seq_id -> Varchar,
        product_price_action_type_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        rate_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_action_type (product_price_action_type_id) {
        product_price_action_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_auto_notice (product_price_notice_id) {
        product_price_notice_id -> Varchar,
        facility_id -> Nullable<Varchar>,
        run_date -> Nullable<Timestamptz>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_change (product_price_change_id) {
        product_price_change_id -> Varchar,
        product_id -> Nullable<Varchar>,
        product_price_type_id -> Nullable<Varchar>,
        product_price_purpose_id -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        product_store_group_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        price -> Nullable<Numeric>,
        old_price -> Nullable<Numeric>,
        changed_date -> Nullable<Timestamptz>,
        changed_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_cond (product_price_rule_id, product_price_cond_seq_id) {
        product_price_rule_id -> Varchar,
        product_price_cond_seq_id -> Varchar,
        input_param_enum_id -> Nullable<Varchar>,
        operator_enum_id -> Nullable<Varchar>,
        cond_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_purpose (product_price_purpose_id) {
        product_price_purpose_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_rule (product_price_rule_id) {
        product_price_rule_id -> Varchar,
        rule_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        is_sale -> Nullable<Bpchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_price_type (product_price_type_id) {
        product_price_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo (product_promo_id) {
        product_promo_id -> Varchar,
        promo_name -> Nullable<Varchar>,
        promo_text -> Nullable<Varchar>,
        user_entered -> Nullable<Bpchar>,
        show_to_customer -> Nullable<Bpchar>,
        require_code -> Nullable<Bpchar>,
        use_limit_per_order -> Nullable<Numeric>,
        use_limit_per_customer -> Nullable<Numeric>,
        use_limit_per_promotion -> Nullable<Numeric>,
        billback_factor -> Nullable<Numeric>,
        override_org_party_id -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_action (product_promo_id, product_promo_rule_id, product_promo_action_seq_id) {
        product_promo_id -> Varchar,
        product_promo_rule_id -> Varchar,
        product_promo_action_seq_id -> Varchar,
        product_promo_action_enum_id -> Nullable<Varchar>,
        custom_method_id -> Nullable<Varchar>,
        order_adjustment_type_id -> Nullable<Varchar>,
        service_name -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        amount -> Nullable<Numeric>,
        product_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        use_cart_quantity -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_category (product_promo_id, product_promo_rule_id, product_promo_action_seq_id, product_promo_cond_seq_id, product_category_id, and_group_id) {
        product_promo_id -> Varchar,
        product_promo_rule_id -> Varchar,
        product_promo_action_seq_id -> Varchar,
        product_promo_cond_seq_id -> Varchar,
        product_category_id -> Varchar,
        and_group_id -> Varchar,
        product_promo_appl_enum_id -> Nullable<Varchar>,
        include_sub_categories -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_code (product_promo_code_id) {
        product_promo_code_id -> Varchar,
        product_promo_id -> Nullable<Varchar>,
        user_entered -> Nullable<Bpchar>,
        require_email_or_party -> Nullable<Bpchar>,
        use_limit_per_code -> Nullable<Numeric>,
        use_limit_per_customer -> Nullable<Numeric>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_code_email (product_promo_code_id, email_address) {
        product_promo_code_id -> Varchar,
        email_address -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_code_party (product_promo_code_id, party_id) {
        product_promo_code_id -> Varchar,
        party_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_cond (product_promo_id, product_promo_rule_id, product_promo_cond_seq_id) {
        product_promo_id -> Varchar,
        product_promo_rule_id -> Varchar,
        product_promo_cond_seq_id -> Varchar,
        custom_method_id -> Nullable<Varchar>,
        input_param_enum_id -> Nullable<Varchar>,
        operator_enum_id -> Nullable<Varchar>,
        cond_value -> Nullable<Varchar>,
        other_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_content (product_promo_id, content_id, product_promo_content_type_id, from_date) {
        product_promo_id -> Varchar,
        content_id -> Varchar,
        product_promo_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_product (product_promo_id, product_promo_rule_id, product_promo_action_seq_id, product_promo_cond_seq_id, product_id) {
        product_promo_id -> Varchar,
        product_promo_rule_id -> Varchar,
        product_promo_action_seq_id -> Varchar,
        product_promo_cond_seq_id -> Varchar,
        product_id -> Varchar,
        product_promo_appl_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_rule (product_promo_id, product_promo_rule_id) {
        product_promo_id -> Varchar,
        product_promo_rule_id -> Varchar,
        rule_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_promo_use (order_id, promo_sequence_id) {
        order_id -> Varchar,
        promo_sequence_id -> Varchar,
        product_promo_id -> Nullable<Varchar>,
        product_promo_code_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        total_discount_amount -> Nullable<Numeric>,
        quantity_left_in_actions -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_review (product_review_id) {
        product_review_id -> Varchar,
        product_store_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        user_login_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        posted_anonymous -> Nullable<Bpchar>,
        posted_date_time -> Nullable<Timestamptz>,
        product_rating -> Nullable<Numeric>,
        #[sql_name = "product_review"]
        product_review_name -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_role (product_id, party_id, role_type_id, from_date) {
        product_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_search_constraint (product_search_result_id, constraint_seq_id) {
        product_search_result_id -> Varchar,
        constraint_seq_id -> Varchar,
        constraint_name -> Nullable<Varchar>,
        info_string -> Nullable<Varchar>,
        include_sub_categories -> Nullable<Bpchar>,
        is_and -> Nullable<Bpchar>,
        any_prefix -> Nullable<Bpchar>,
        any_suffix -> Nullable<Bpchar>,
        remove_stems -> Nullable<Bpchar>,
        low_value -> Nullable<Varchar>,
        high_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_search_result (product_search_result_id) {
        product_search_result_id -> Varchar,
        visit_id -> Nullable<Varchar>,
        order_by_name -> Nullable<Varchar>,
        is_ascending -> Nullable<Bpchar>,
        num_results -> Nullable<Numeric>,
        seconds_total -> Nullable<Float8>,
        search_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store (product_store_id) {
        product_store_id -> Varchar,
        primary_store_group_id -> Nullable<Varchar>,
        store_name -> Nullable<Varchar>,
        company_name -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        subtitle -> Nullable<Varchar>,
        pay_to_party_id -> Nullable<Varchar>,
        days_to_cancel_non_pay -> Nullable<Numeric>,
        manual_auth_is_capture -> Nullable<Bpchar>,
        prorate_shipping -> Nullable<Bpchar>,
        prorate_taxes -> Nullable<Bpchar>,
        view_cart_on_add -> Nullable<Bpchar>,
        auto_save_cart -> Nullable<Bpchar>,
        auto_approve_reviews -> Nullable<Bpchar>,
        is_demo_store -> Nullable<Bpchar>,
        is_immediately_fulfilled -> Nullable<Bpchar>,
        inventory_facility_id -> Nullable<Varchar>,
        one_inventory_facility -> Nullable<Bpchar>,
        check_inventory -> Nullable<Bpchar>,
        reserve_inventory -> Nullable<Bpchar>,
        reserve_order_enum_id -> Nullable<Varchar>,
        require_inventory -> Nullable<Bpchar>,
        balance_res_on_order_creation -> Nullable<Bpchar>,
        requirement_method_enum_id -> Nullable<Varchar>,
        order_number_prefix -> Nullable<Varchar>,
        default_locale_string -> Nullable<Varchar>,
        default_currency_uom_id -> Nullable<Varchar>,
        default_time_zone_string -> Nullable<Varchar>,
        default_sales_channel_enum_id -> Nullable<Varchar>,
        allow_password -> Nullable<Bpchar>,
        default_password -> Nullable<Varchar>,
        explode_order_items -> Nullable<Bpchar>,
        check_gc_balance -> Nullable<Bpchar>,
        retry_failed_auths -> Nullable<Bpchar>,
        header_approved_status -> Nullable<Varchar>,
        item_approved_status -> Nullable<Varchar>,
        digital_item_approved_status -> Nullable<Varchar>,
        header_declined_status -> Nullable<Varchar>,
        item_declined_status -> Nullable<Varchar>,
        header_cancel_status -> Nullable<Varchar>,
        item_cancel_status -> Nullable<Varchar>,
        auth_declined_message -> Nullable<Varchar>,
        auth_fraud_message -> Nullable<Varchar>,
        auth_error_message -> Nullable<Varchar>,
        visual_theme_id -> Nullable<Varchar>,
        store_credit_account_enum_id -> Nullable<Varchar>,
        use_primary_email_username -> Nullable<Bpchar>,
        require_customer_role -> Nullable<Bpchar>,
        auto_invoice_digital_items -> Nullable<Bpchar>,
        req_ship_addr_for_dig_items -> Nullable<Bpchar>,
        show_checkout_gift_options -> Nullable<Bpchar>,
        select_payment_type_per_item -> Nullable<Bpchar>,
        show_prices_with_vat_tax -> Nullable<Bpchar>,
        show_tax_is_exempt -> Nullable<Bpchar>,
        vat_tax_auth_geo_id -> Nullable<Varchar>,
        vat_tax_auth_party_id -> Nullable<Varchar>,
        enable_auto_suggestion_list -> Nullable<Bpchar>,
        enable_dig_prod_upload -> Nullable<Bpchar>,
        prod_search_exclude_variants -> Nullable<Bpchar>,
        dig_prod_upload_category_id -> Nullable<Varchar>,
        auto_order_cc_try_exp -> Nullable<Bpchar>,
        auto_order_cc_try_other_cards -> Nullable<Bpchar>,
        auto_order_cc_try_later_nsf -> Nullable<Bpchar>,
        auto_order_cc_try_later_max -> Nullable<Numeric>,
        store_credit_valid_days -> Nullable<Numeric>,
        auto_approve_invoice -> Nullable<Bpchar>,
        auto_approve_order -> Nullable<Bpchar>,
        ship_if_capture_fails -> Nullable<Bpchar>,
        set_owner_upon_issuance -> Nullable<Bpchar>,
        req_return_inventory_receive -> Nullable<Bpchar>,
        add_to_cart_remove_incompat -> Nullable<Bpchar>,
        add_to_cart_replace_upsell -> Nullable<Bpchar>,
        split_pay_pref_per_shp_grp -> Nullable<Bpchar>,
        managed_by_lot -> Nullable<Bpchar>,
        show_out_of_stock_products -> Nullable<Bpchar>,
        order_decimal_quantity -> Nullable<Bpchar>,
        allow_comment -> Nullable<Bpchar>,
        allocate_inventory -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_catalog (product_store_id, prod_catalog_id, from_date) {
        product_store_id -> Varchar,
        prod_catalog_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_email_setting (product_store_id, email_type) {
        product_store_id -> Varchar,
        email_type -> Varchar,
        body_screen_location -> Nullable<Varchar>,
        xslfo_attach_screen_location -> Nullable<Varchar>,
        from_address -> Nullable<Varchar>,
        cc_address -> Nullable<Varchar>,
        bcc_address -> Nullable<Varchar>,
        subject -> Nullable<Varchar>,
        content_type -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_facility (product_store_id, facility_id, from_date) {
        product_store_id -> Varchar,
        facility_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_fin_act_setting (product_store_id, fin_account_type_id) {
        product_store_id -> Varchar,
        fin_account_type_id -> Varchar,
        require_pin_code -> Nullable<Bpchar>,
        validate_g_c_fin_acct -> Nullable<Bpchar>,
        account_code_length -> Nullable<Numeric>,
        pin_code_length -> Nullable<Numeric>,
        account_valid_days -> Nullable<Numeric>,
        auth_valid_days -> Nullable<Numeric>,
        purchase_survey_id -> Nullable<Varchar>,
        purch_survey_send_to -> Nullable<Varchar>,
        purch_survey_copy_me -> Nullable<Varchar>,
        allow_auth_to_negative -> Nullable<Bpchar>,
        min_balance -> Nullable<Numeric>,
        replenish_threshold -> Nullable<Numeric>,
        replenish_method_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_group (product_store_group_id) {
        product_store_group_id -> Varchar,
        product_store_group_type_id -> Nullable<Varchar>,
        primary_parent_group_id -> Nullable<Varchar>,
        product_store_group_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_group_member (product_store_id, product_store_group_id, from_date) {
        product_store_id -> Varchar,
        product_store_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_group_role (product_store_group_id, party_id, role_type_id) {
        product_store_group_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_group_rollup (product_store_group_id, parent_group_id, from_date) {
        product_store_group_id -> Varchar,
        parent_group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_group_type (product_store_group_type_id) {
        product_store_group_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_keyword_ovrd (product_store_id, keyword, from_date) {
        product_store_id -> Varchar,
        keyword -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        target -> Nullable<Varchar>,
        target_type_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_payment_setting (product_store_id, payment_method_type_id, payment_service_type_enum_id) {
        product_store_id -> Varchar,
        payment_method_type_id -> Varchar,
        payment_service_type_enum_id -> Varchar,
        payment_service -> Nullable<Varchar>,
        payment_custom_method_id -> Nullable<Varchar>,
        payment_gateway_config_id -> Nullable<Varchar>,
        payment_properties_path -> Nullable<Varchar>,
        apply_to_all_products -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_promo_appl (product_store_id, product_promo_id, from_date) {
        product_store_id -> Varchar,
        product_promo_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        manual_only -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_role (party_id, role_type_id, product_store_id, from_date) {
        party_id -> Varchar,
        role_type_id -> Varchar,
        product_store_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_shipment_meth (product_store_ship_meth_id) {
        product_store_ship_meth_id -> Varchar,
        product_store_id -> Nullable<Varchar>,
        shipment_method_type_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        company_party_id -> Nullable<Varchar>,
        min_weight -> Nullable<Numeric>,
        max_weight -> Nullable<Numeric>,
        min_size -> Nullable<Numeric>,
        max_size -> Nullable<Numeric>,
        min_total -> Nullable<Numeric>,
        max_total -> Nullable<Numeric>,
        allow_usps_addr -> Nullable<Bpchar>,
        require_usps_addr -> Nullable<Bpchar>,
        allow_company_addr -> Nullable<Bpchar>,
        require_company_addr -> Nullable<Bpchar>,
        include_no_charge_items -> Nullable<Bpchar>,
        include_feature_group -> Nullable<Varchar>,
        exclude_feature_group -> Nullable<Varchar>,
        include_geo_id -> Nullable<Varchar>,
        exclude_geo_id -> Nullable<Varchar>,
        service_name -> Nullable<Varchar>,
        config_props -> Nullable<Varchar>,
        shipment_custom_method_id -> Nullable<Varchar>,
        shipment_gateway_config_id -> Nullable<Varchar>,
        sequence_number -> Nullable<Numeric>,
        allowance_percent -> Nullable<Numeric>,
        minimum_price -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_survey_appl (product_store_survey_id) {
        product_store_survey_id -> Varchar,
        product_store_id -> Nullable<Varchar>,
        survey_appl_type_id -> Nullable<Varchar>,
        group_name -> Nullable<Varchar>,
        survey_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        product_category_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        survey_template -> Nullable<Varchar>,
        result_template -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_telecom_setting (product_store_id, telecom_method_type_id, telecom_msg_type_enum_id) {
        product_store_id -> Varchar,
        telecom_method_type_id -> Varchar,
        telecom_msg_type_enum_id -> Varchar,
        telecom_custom_method_id -> Nullable<Varchar>,
        telecom_gateway_config_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_vendor_payment (product_store_id, vendor_party_id, payment_method_type_id, credit_card_enum_id) {
        product_store_id -> Varchar,
        vendor_party_id -> Varchar,
        payment_method_type_id -> Varchar,
        credit_card_enum_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_store_vendor_shipment (product_store_id, vendor_party_id, shipment_method_type_id, carrier_party_id) {
        product_store_id -> Varchar,
        vendor_party_id -> Varchar,
        shipment_method_type_id -> Varchar,
        carrier_party_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_subscription_resource (product_id, subscription_resource_id, from_date) {
        product_id -> Varchar,
        subscription_resource_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        purchase_from_date -> Nullable<Timestamptz>,
        purchase_thru_date -> Nullable<Timestamptz>,
        max_life_time -> Nullable<Numeric>,
        max_life_time_uom_id -> Nullable<Varchar>,
        available_time -> Nullable<Numeric>,
        available_time_uom_id -> Nullable<Varchar>,
        use_count_limit -> Nullable<Numeric>,
        use_time -> Nullable<Numeric>,
        use_time_uom_id -> Nullable<Varchar>,
        use_role_type_id -> Nullable<Varchar>,
        automatic_extend -> Nullable<Bpchar>,
        cancl_autm_ext_time -> Nullable<Numeric>,
        cancl_autm_ext_time_uom_id -> Nullable<Varchar>,
        grace_period_on_expiry -> Nullable<Numeric>,
        grace_period_on_expiry_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_type (product_type_id) {
        product_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        is_physical -> Nullable<Bpchar>,
        is_digital -> Nullable<Bpchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    product_type_attr (product_type_id, attr_name) {
        product_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    protected_view (group_id, view_name_id) {
        group_id -> Varchar,
        view_name_id -> Varchar,
        max_hits -> Nullable<Numeric>,
        max_hits_duration -> Nullable<Numeric>,
        tarpit_duration -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    protocol_type (protocol_type_id) {
        protocol_type_id -> Varchar,
        protocol_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quantity_break (quantity_break_id) {
        quantity_break_id -> Varchar,
        quantity_break_type_id -> Nullable<Varchar>,
        from_quantity -> Nullable<Numeric>,
        thru_quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quantity_break_type (quantity_break_type_id) {
        quantity_break_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote (quote_id) {
        quote_id -> Varchar,
        quote_type_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        issue_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        sales_channel_enum_id -> Nullable<Varchar>,
        valid_from_date -> Nullable<Timestamptz>,
        valid_thru_date -> Nullable<Timestamptz>,
        quote_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_adjustment (quote_adjustment_id) {
        quote_adjustment_id -> Varchar,
        quote_adjustment_type_id -> Nullable<Varchar>,
        quote_id -> Nullable<Varchar>,
        quote_item_seq_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        product_promo_id -> Nullable<Varchar>,
        product_promo_rule_id -> Nullable<Varchar>,
        product_promo_action_seq_id -> Nullable<Varchar>,
        product_feature_id -> Nullable<Varchar>,
        corresponding_product_id -> Nullable<Varchar>,
        source_reference_id -> Nullable<Varchar>,
        source_percentage -> Nullable<Numeric>,
        customer_reference_id -> Nullable<Varchar>,
        primary_geo_id -> Nullable<Varchar>,
        secondary_geo_id -> Nullable<Varchar>,
        exempt_amount -> Nullable<Numeric>,
        tax_auth_geo_id -> Nullable<Varchar>,
        tax_auth_party_id -> Nullable<Varchar>,
        override_gl_account_id -> Nullable<Varchar>,
        include_in_tax -> Nullable<Bpchar>,
        include_in_shipping -> Nullable<Bpchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_attribute (quote_id, attr_name) {
        quote_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_coefficient (quote_id, coeff_name) {
        quote_id -> Varchar,
        coeff_name -> Varchar,
        coeff_value -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_item (quote_id, quote_item_seq_id) {
        quote_id -> Varchar,
        quote_item_seq_id -> Varchar,
        product_id -> Nullable<Varchar>,
        product_feature_id -> Nullable<Varchar>,
        deliverable_type_id -> Nullable<Varchar>,
        skill_type_id -> Nullable<Varchar>,
        uom_id -> Nullable<Varchar>,
        work_effort_id -> Nullable<Varchar>,
        cust_request_id -> Nullable<Varchar>,
        cust_request_item_seq_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        selected_amount -> Nullable<Numeric>,
        quote_unit_price -> Nullable<Numeric>,
        reserv_start -> Nullable<Timestamptz>,
        reserv_length -> Nullable<Numeric>,
        reserv_persons -> Nullable<Numeric>,
        config_id -> Nullable<Varchar>,
        estimated_delivery_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        is_promo -> Nullable<Bpchar>,
        lead_time_days -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_note (quote_id, note_id) {
        quote_id -> Varchar,
        note_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_role (quote_id, party_id, role_type_id) {
        quote_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_term (term_type_id, quote_id, quote_item_seq_id) {
        term_type_id -> Varchar,
        quote_id -> Varchar,
        quote_item_seq_id -> Varchar,
        term_value -> Nullable<Numeric>,
        uom_id -> Nullable<Varchar>,
        term_days -> Nullable<Numeric>,
        text_value -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_term_attribute (term_type_id, quote_id, quote_item_seq_id, attr_name) {
        term_type_id -> Varchar,
        quote_id -> Varchar,
        quote_item_seq_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_type (quote_type_id) {
        quote_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_type_attr (quote_type_id, attr_name) {
        quote_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    quote_work_effort (quote_id, work_effort_id) {
        quote_id -> Varchar,
        work_effort_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    rate_amount (rate_type_id, rate_currency_uom_id, period_type_id, party_id, work_effort_id, empl_position_type_id, from_date) {
        rate_type_id -> Varchar,
        rate_currency_uom_id -> Varchar,
        period_type_id -> Varchar,
        work_effort_id -> Varchar,
        party_id -> Varchar,
        empl_position_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        #[sql_name = "rate_amount"]
        rate_amount_value -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    rate_type (rate_type_id) {
        rate_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    recurrence_info (recurrence_info_id) {
        recurrence_info_id -> Varchar,
        start_date_time -> Nullable<Timestamptz>,
        exception_date_times -> Nullable<Text>,
        recurrence_date_times -> Nullable<Text>,
        exception_rule_id -> Nullable<Varchar>,
        recurrence_rule_id -> Nullable<Varchar>,
        recurrence_count -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    recurrence_rule (recurrence_rule_id) {
        recurrence_rule_id -> Varchar,
        frequency -> Nullable<Varchar>,
        until_date_time -> Nullable<Timestamptz>,
        count_number -> Nullable<Numeric>,
        interval_number -> Nullable<Numeric>,
        by_second_list -> Nullable<Text>,
        by_minute_list -> Nullable<Text>,
        by_hour_list -> Nullable<Text>,
        by_day_list -> Nullable<Text>,
        by_month_day_list -> Nullable<Text>,
        by_year_day_list -> Nullable<Text>,
        by_week_no_list -> Nullable<Text>,
        by_month_list -> Nullable<Text>,
        by_set_pos_list -> Nullable<Text>,
        week_start -> Nullable<Varchar>,
        x_name -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    rejection_reason (rejection_id) {
        rejection_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    reorder_guideline (reorder_guideline_id) {
        reorder_guideline_id -> Varchar,
        product_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        geo_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        reorder_quantity -> Nullable<Numeric>,
        reorder_level -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement (requirement_id) {
        requirement_id -> Varchar,
        requirement_type_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        deliverable_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        requirement_start_date -> Nullable<Timestamptz>,
        required_by_date -> Nullable<Timestamptz>,
        estimated_budget -> Nullable<Numeric>,
        quantity -> Nullable<Numeric>,
        use_case -> Nullable<Text>,
        reason -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        facility_id_to -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement_attribute (requirement_id, attr_name) {
        requirement_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement_budget_allocation (budget_id, budget_item_seq_id, requirement_id) {
        budget_id -> Varchar,
        budget_item_seq_id -> Varchar,
        requirement_id -> Varchar,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement_cust_request (cust_request_id, cust_request_item_seq_id, requirement_id) {
        cust_request_id -> Varchar,
        cust_request_item_seq_id -> Varchar,
        requirement_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement_role (requirement_id, party_id, role_type_id, from_date) {
        requirement_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement_status (requirement_id, status_id) {
        requirement_id -> Varchar,
        status_id -> Varchar,
        status_date -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement_type (requirement_type_id) {
        requirement_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    requirement_type_attr (requirement_type_id, attr_name) {
        requirement_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    responding_party (responding_party_seq_id, cust_request_id, party_id) {
        responding_party_seq_id -> Varchar,
        cust_request_id -> Varchar,
        party_id -> Varchar,
        contact_mech_id -> Nullable<Varchar>,
        date_sent -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    responsibility_type (responsibility_type_id) {
        responsibility_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_adjustment (return_adjustment_id) {
        return_adjustment_id -> Varchar,
        return_adjustment_type_id -> Nullable<Varchar>,
        return_id -> Nullable<Varchar>,
        return_item_seq_id -> Nullable<Varchar>,
        ship_group_seq_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        return_type_id -> Nullable<Varchar>,
        order_adjustment_id -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        product_promo_id -> Nullable<Varchar>,
        product_promo_rule_id -> Nullable<Varchar>,
        product_promo_action_seq_id -> Nullable<Varchar>,
        product_feature_id -> Nullable<Varchar>,
        corresponding_product_id -> Nullable<Varchar>,
        tax_authority_rate_seq_id -> Nullable<Varchar>,
        source_reference_id -> Nullable<Varchar>,
        source_percentage -> Nullable<Numeric>,
        customer_reference_id -> Nullable<Varchar>,
        primary_geo_id -> Nullable<Varchar>,
        secondary_geo_id -> Nullable<Varchar>,
        exempt_amount -> Nullable<Numeric>,
        tax_auth_geo_id -> Nullable<Varchar>,
        tax_auth_party_id -> Nullable<Varchar>,
        override_gl_account_id -> Nullable<Varchar>,
        include_in_tax -> Nullable<Bpchar>,
        include_in_shipping -> Nullable<Bpchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_adjustment_type (return_adjustment_type_id) {
        return_adjustment_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_contact_mech (return_id, contact_mech_purpose_type_id, contact_mech_id) {
        return_id -> Varchar,
        contact_mech_purpose_type_id -> Varchar,
        contact_mech_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_header (return_id) {
        return_id -> Varchar,
        return_header_type_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        created_by -> Nullable<Varchar>,
        from_party_id -> Nullable<Varchar>,
        to_party_id -> Nullable<Varchar>,
        payment_method_id -> Nullable<Varchar>,
        fin_account_id -> Nullable<Varchar>,
        billing_account_id -> Nullable<Varchar>,
        entry_date -> Nullable<Timestamptz>,
        origin_contact_mech_id -> Nullable<Varchar>,
        destination_facility_id -> Nullable<Varchar>,
        needs_inventory_receive -> Nullable<Bpchar>,
        currency_uom_id -> Nullable<Varchar>,
        supplier_rma_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_header_type (return_header_type_id) {
        return_header_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_item (return_id, return_item_seq_id) {
        return_id -> Varchar,
        return_item_seq_id -> Varchar,
        return_reason_id -> Nullable<Varchar>,
        return_type_id -> Nullable<Varchar>,
        return_item_type_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        expected_item_status -> Nullable<Varchar>,
        return_quantity -> Nullable<Numeric>,
        received_quantity -> Nullable<Numeric>,
        return_price -> Nullable<Numeric>,
        return_item_response_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_item_billing (return_id, return_item_seq_id, invoice_id, invoice_item_seq_id) {
        return_id -> Varchar,
        return_item_seq_id -> Varchar,
        invoice_id -> Varchar,
        invoice_item_seq_id -> Varchar,
        shipment_receipt_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        amount -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_item_response (return_item_response_id) {
        return_item_response_id -> Varchar,
        order_payment_preference_id -> Nullable<Varchar>,
        replacement_order_id -> Nullable<Varchar>,
        payment_id -> Nullable<Varchar>,
        billing_account_id -> Nullable<Varchar>,
        fin_account_trans_id -> Nullable<Varchar>,
        response_amount -> Nullable<Numeric>,
        response_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_item_shipment (return_id, return_item_seq_id, shipment_id, shipment_item_seq_id) {
        return_id -> Varchar,
        return_item_seq_id -> Varchar,
        shipment_id -> Varchar,
        shipment_item_seq_id -> Varchar,
        quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_item_type (return_item_type_id) {
        return_item_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_item_type_map (return_item_map_key, return_header_type_id) {
        return_item_map_key -> Varchar,
        return_header_type_id -> Varchar,
        return_item_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_reason (return_reason_id) {
        return_reason_id -> Varchar,
        description -> Nullable<Varchar>,
        sequence_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_status (return_status_id) {
        return_status_id -> Varchar,
        status_id -> Nullable<Varchar>,
        return_id -> Nullable<Varchar>,
        return_item_seq_id -> Nullable<Varchar>,
        change_by_user_login_id -> Nullable<Varchar>,
        status_datetime -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    return_type (return_type_id) {
        return_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        sequence_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    role_type (role_type_id) {
        role_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    role_type_attr (role_type_id, attr_name) {
        role_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    runtime_data (runtime_data_id) {
        runtime_data_id -> Varchar,
        runtime_info -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    salary_step_new (salary_step_seq_id, pay_grade_id, from_date) {
        salary_step_seq_id -> Varchar,
        pay_grade_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        date_modified -> Nullable<Timestamptz>,
        amount -> Nullable<Numeric>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sale_type (sale_type_id) {
        sale_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_forecast (sales_forecast_id) {
        sales_forecast_id -> Varchar,
        parent_sales_forecast_id -> Nullable<Varchar>,
        organization_party_id -> Nullable<Varchar>,
        internal_party_id -> Nullable<Varchar>,
        custom_time_period_id -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        quota_amount -> Nullable<Numeric>,
        forecast_amount -> Nullable<Numeric>,
        best_case_amount -> Nullable<Numeric>,
        closed_amount -> Nullable<Numeric>,
        percent_of_quota_forecast -> Nullable<Numeric>,
        percent_of_quota_closed -> Nullable<Numeric>,
        pipeline_amount -> Nullable<Numeric>,
        created_by_user_login_id -> Nullable<Varchar>,
        modified_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_forecast_detail (sales_forecast_id, sales_forecast_detail_id) {
        sales_forecast_id -> Varchar,
        sales_forecast_detail_id -> Varchar,
        amount -> Nullable<Numeric>,
        quantity_uom_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        product_id -> Nullable<Varchar>,
        product_category_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_forecast_history (sales_forecast_history_id) {
        sales_forecast_history_id -> Varchar,
        sales_forecast_id -> Nullable<Varchar>,
        parent_sales_forecast_id -> Nullable<Varchar>,
        organization_party_id -> Nullable<Varchar>,
        internal_party_id -> Nullable<Varchar>,
        custom_time_period_id -> Nullable<Varchar>,
        currency_uom_id -> Nullable<Varchar>,
        quota_amount -> Nullable<Numeric>,
        forecast_amount -> Nullable<Numeric>,
        best_case_amount -> Nullable<Numeric>,
        closed_amount -> Nullable<Numeric>,
        percent_of_quota_forecast -> Nullable<Numeric>,
        percent_of_quota_closed -> Nullable<Numeric>,
        change_note -> Nullable<Text>,
        modified_by_user_login_id -> Nullable<Varchar>,
        modified_timestamp -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity (sales_opportunity_id) {
        sales_opportunity_id -> Varchar,
        opportunity_name -> Nullable<Varchar>,
        description -> Nullable<Text>,
        next_step -> Nullable<Text>,
        next_step_date -> Nullable<Timestamptz>,
        estimated_amount -> Nullable<Numeric>,
        estimated_probability -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        marketing_campaign_id -> Nullable<Varchar>,
        data_source_id -> Nullable<Varchar>,
        estimated_close_date -> Nullable<Timestamptz>,
        opportunity_stage_id -> Nullable<Varchar>,
        type_enum_id -> Nullable<Varchar>,
        created_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity_competitor (sales_opportunity_id, competitor_party_id) {
        sales_opportunity_id -> Varchar,
        competitor_party_id -> Varchar,
        position_enum_id -> Nullable<Varchar>,
        strengths -> Nullable<Text>,
        weaknesses -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity_history (sales_opportunity_history_id) {
        sales_opportunity_history_id -> Varchar,
        sales_opportunity_id -> Nullable<Varchar>,
        description -> Nullable<Text>,
        next_step -> Nullable<Text>,
        estimated_amount -> Nullable<Numeric>,
        estimated_probability -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        estimated_close_date -> Nullable<Timestamptz>,
        opportunity_stage_id -> Nullable<Varchar>,
        change_note -> Nullable<Text>,
        modified_by_user_login -> Nullable<Varchar>,
        modified_timestamp -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity_quote (sales_opportunity_id, quote_id) {
        sales_opportunity_id -> Varchar,
        quote_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity_role (sales_opportunity_id, party_id, role_type_id) {
        sales_opportunity_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity_stage (opportunity_stage_id) {
        opportunity_stage_id -> Varchar,
        description -> Nullable<Varchar>,
        default_probability -> Nullable<Numeric>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity_trck_code (sales_opportunity_id, tracking_code_id) {
        sales_opportunity_id -> Varchar,
        tracking_code_id -> Varchar,
        received_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sales_opportunity_work_effort (sales_opportunity_id, work_effort_id) {
        sales_opportunity_id -> Varchar,
        work_effort_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    security_group (group_id) {
        group_id -> Varchar,
        group_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    security_group_permission (group_id, permission_id, from_date) {
        group_id -> Varchar,
        permission_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    security_permission (permission_id) {
        permission_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    segment_group (segment_group_id) {
        segment_group_id -> Varchar,
        segment_group_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    segment_group_classification (segment_group_id, party_classification_group_id) {
        segment_group_id -> Varchar,
        party_classification_group_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    segment_group_geo (segment_group_id, geo_id) {
        segment_group_id -> Varchar,
        geo_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    segment_group_role (segment_group_id, party_id, role_type_id) {
        segment_group_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    segment_group_type (segment_group_type_id) {
        segment_group_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    sequence_value_item (seq_name) {
        seq_name -> Varchar,
        seq_id -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    server_hit (visit_id, content_id, hit_start_date_time, hit_type_id) {
        visit_id -> Varchar,
        content_id -> Varchar,
        hit_start_date_time -> Timestamptz,
        hit_type_id -> Varchar,
        num_of_bytes -> Nullable<Numeric>,
        running_time_millis -> Nullable<Numeric>,
        user_login_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        request_url -> Nullable<Varchar>,
        referrer_url -> Nullable<Varchar>,
        server_ip_address -> Nullable<Varchar>,
        server_host_name -> Nullable<Varchar>,
        internal_content_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        id_by_ip_contact_mech_id -> Nullable<Varchar>,
        ref_by_web_contact_mech_id -> Nullable<Varchar>,
    }
}

table! {
    server_hit_bin (server_hit_bin_id) {
        server_hit_bin_id -> Varchar,
        content_id -> Nullable<Varchar>,
        hit_type_id -> Nullable<Varchar>,
        server_ip_address -> Nullable<Varchar>,
        server_host_name -> Nullable<Varchar>,
        bin_start_date_time -> Nullable<Timestamptz>,
        bin_end_date_time -> Nullable<Timestamptz>,
        number_hits -> Nullable<Numeric>,
        total_time_millis -> Nullable<Numeric>,
        min_time_millis -> Nullable<Numeric>,
        max_time_millis -> Nullable<Numeric>,
        internal_content_id -> Nullable<Varchar>,
    }
}

table! {
    server_hit_type (hit_type_id) {
        hit_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    service_semaphore (service_name) {
        service_name -> Varchar,
        locked_by_instance_id -> Nullable<Varchar>,
        lock_thread -> Nullable<Varchar>,
        lock_time -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    settlement_term (settlement_term_id) {
        settlement_term_id -> Varchar,
        term_name -> Nullable<Varchar>,
        term_value -> Nullable<Numeric>,
        uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment (shipment_id) {
        shipment_id -> Varchar,
        shipment_type_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        primary_order_id -> Nullable<Varchar>,
        primary_return_id -> Nullable<Varchar>,
        primary_ship_group_seq_id -> Nullable<Varchar>,
        picklist_bin_id -> Nullable<Varchar>,
        estimated_ready_date -> Nullable<Timestamptz>,
        estimated_ship_date -> Nullable<Timestamptz>,
        estimated_ship_work_eff_id -> Nullable<Varchar>,
        estimated_arrival_date -> Nullable<Timestamptz>,
        estimated_arrival_work_eff_id -> Nullable<Varchar>,
        latest_cancel_date -> Nullable<Timestamptz>,
        estimated_ship_cost -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        handling_instructions -> Nullable<Varchar>,
        origin_facility_id -> Nullable<Varchar>,
        destination_facility_id -> Nullable<Varchar>,
        origin_contact_mech_id -> Nullable<Varchar>,
        origin_telecom_number_id -> Nullable<Varchar>,
        destination_contact_mech_id -> Nullable<Varchar>,
        destination_telecom_number_id -> Nullable<Varchar>,
        party_id_to -> Nullable<Varchar>,
        party_id_from -> Nullable<Varchar>,
        additional_shipping_charge -> Nullable<Numeric>,
        addtl_shipping_charge_desc -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_attribute (shipment_id, attr_name) {
        shipment_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_box_type (shipment_box_type_id) {
        shipment_box_type_id -> Varchar,
        description -> Nullable<Varchar>,
        dimension_uom_id -> Nullable<Varchar>,
        box_length -> Nullable<Numeric>,
        box_width -> Nullable<Numeric>,
        box_height -> Nullable<Numeric>,
        weight_uom_id -> Nullable<Varchar>,
        box_weight -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_contact_mech (shipment_id, shipment_contact_mech_type_id) {
        shipment_id -> Varchar,
        shipment_contact_mech_type_id -> Varchar,
        contact_mech_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_contact_mech_type (shipment_contact_mech_type_id) {
        shipment_contact_mech_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_cost_estimate (shipment_cost_estimate_id) {
        shipment_cost_estimate_id -> Varchar,
        shipment_method_type_id -> Nullable<Varchar>,
        carrier_party_id -> Nullable<Varchar>,
        carrier_role_type_id -> Nullable<Varchar>,
        product_store_ship_meth_id -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        geo_id_to -> Nullable<Varchar>,
        geo_id_from -> Nullable<Varchar>,
        weight_break_id -> Nullable<Varchar>,
        weight_uom_id -> Nullable<Varchar>,
        weight_unit_price -> Nullable<Numeric>,
        quantity_break_id -> Nullable<Varchar>,
        quantity_uom_id -> Nullable<Varchar>,
        quantity_unit_price -> Nullable<Numeric>,
        price_break_id -> Nullable<Varchar>,
        price_uom_id -> Nullable<Varchar>,
        price_unit_price -> Nullable<Numeric>,
        order_flat_price -> Nullable<Numeric>,
        order_price_percent -> Nullable<Numeric>,
        order_item_flat_price -> Nullable<Numeric>,
        shipping_price_percent -> Nullable<Numeric>,
        product_feature_group_id -> Nullable<Varchar>,
        oversize_unit -> Nullable<Numeric>,
        oversize_price -> Nullable<Numeric>,
        feature_percent -> Nullable<Numeric>,
        feature_price -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_gateway_config (shipment_gateway_config_id) {
        shipment_gateway_config_id -> Varchar,
        shipment_gateway_conf_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_gateway_config_type (shipment_gateway_conf_type_id) {
        shipment_gateway_conf_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_gateway_dhl (shipment_gateway_config_id) {
        shipment_gateway_config_id -> Varchar,
        connect_url -> Nullable<Varchar>,
        connect_timeout -> Nullable<Numeric>,
        head_version -> Nullable<Varchar>,
        head_action -> Nullable<Varchar>,
        access_user_id -> Nullable<Varchar>,
        access_password -> Nullable<Varchar>,
        access_account_nbr -> Nullable<Varchar>,
        access_shipping_key -> Nullable<Varchar>,
        label_image_format -> Nullable<Varchar>,
        rate_estimate_template -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_gateway_fedex (shipment_gateway_config_id) {
        shipment_gateway_config_id -> Varchar,
        connect_url -> Nullable<Varchar>,
        connect_soap_url -> Nullable<Varchar>,
        connect_timeout -> Nullable<Numeric>,
        access_account_nbr -> Nullable<Varchar>,
        access_meter_number -> Nullable<Varchar>,
        access_user_key -> Nullable<Varchar>,
        access_user_pwd -> Nullable<Varchar>,
        label_image_type -> Nullable<Varchar>,
        default_dropoff_type -> Nullable<Varchar>,
        default_packaging_type -> Nullable<Varchar>,
        template_shipment -> Nullable<Varchar>,
        template_subscription -> Nullable<Varchar>,
        rate_estimate_template -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_gateway_ups (shipment_gateway_config_id) {
        shipment_gateway_config_id -> Varchar,
        connect_url -> Nullable<Varchar>,
        connect_timeout -> Nullable<Numeric>,
        shipper_number -> Nullable<Varchar>,
        bill_shipper_account_number -> Nullable<Varchar>,
        access_license_number -> Nullable<Varchar>,
        access_user_id -> Nullable<Varchar>,
        access_password -> Nullable<Varchar>,
        save_cert_info -> Nullable<Varchar>,
        save_cert_path -> Nullable<Varchar>,
        shipper_pickup_type -> Nullable<Varchar>,
        customer_classification -> Nullable<Varchar>,
        max_estimate_weight -> Nullable<Numeric>,
        min_estimate_weight -> Nullable<Numeric>,
        cod_allow_cod -> Nullable<Varchar>,
        cod_surcharge_amount -> Nullable<Numeric>,
        cod_surcharge_currency_uom_id -> Nullable<Varchar>,
        cod_surcharge_apply_to_package -> Nullable<Varchar>,
        cod_funds_code -> Nullable<Varchar>,
        default_return_label_memo -> Nullable<Varchar>,
        default_return_label_subject -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_gateway_usps (shipment_gateway_config_id) {
        shipment_gateway_config_id -> Varchar,
        connect_url -> Nullable<Varchar>,
        connect_url_labels -> Nullable<Varchar>,
        connect_timeout -> Nullable<Numeric>,
        access_user_id -> Nullable<Varchar>,
        access_password -> Nullable<Varchar>,
        max_estimate_weight -> Nullable<Numeric>,
        test -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_item (shipment_id, shipment_item_seq_id) {
        shipment_id -> Varchar,
        shipment_item_seq_id -> Varchar,
        product_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        shipment_content_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_item_billing (shipment_id, shipment_item_seq_id, invoice_id, invoice_item_seq_id) {
        shipment_id -> Varchar,
        shipment_item_seq_id -> Varchar,
        invoice_id -> Varchar,
        invoice_item_seq_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_item_feature (shipment_id, shipment_item_seq_id, product_feature_id) {
        shipment_id -> Varchar,
        shipment_item_seq_id -> Varchar,
        product_feature_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_method_type (shipment_method_type_id) {
        shipment_method_type_id -> Varchar,
        description -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_package (shipment_id, shipment_package_seq_id) {
        shipment_id -> Varchar,
        shipment_package_seq_id -> Varchar,
        shipment_box_type_id -> Nullable<Varchar>,
        date_created -> Nullable<Timestamptz>,
        box_length -> Nullable<Numeric>,
        box_height -> Nullable<Numeric>,
        box_width -> Nullable<Numeric>,
        dimension_uom_id -> Nullable<Varchar>,
        weight -> Nullable<Numeric>,
        weight_uom_id -> Nullable<Varchar>,
        insured_value -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_package_content (shipment_id, shipment_package_seq_id, shipment_item_seq_id) {
        shipment_id -> Varchar,
        shipment_package_seq_id -> Varchar,
        shipment_item_seq_id -> Varchar,
        quantity -> Nullable<Numeric>,
        sub_product_id -> Nullable<Varchar>,
        sub_product_quantity -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_package_route_seg (shipment_id, shipment_package_seq_id, shipment_route_segment_id) {
        shipment_id -> Varchar,
        shipment_package_seq_id -> Varchar,
        shipment_route_segment_id -> Varchar,
        tracking_code -> Nullable<Varchar>,
        box_number -> Nullable<Varchar>,
        label_image -> Nullable<Bytea>,
        label_intl_sign_image -> Nullable<Bytea>,
        label_html -> Nullable<Text>,
        label_printed -> Nullable<Bpchar>,
        international_invoice -> Nullable<Bytea>,
        package_transport_cost -> Nullable<Numeric>,
        package_service_cost -> Nullable<Numeric>,
        package_other_cost -> Nullable<Numeric>,
        cod_amount -> Nullable<Numeric>,
        insured_amount -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_receipt (receipt_id) {
        receipt_id -> Varchar,
        inventory_item_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        shipment_id -> Nullable<Varchar>,
        shipment_item_seq_id -> Nullable<Varchar>,
        shipment_package_seq_id -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        return_id -> Nullable<Varchar>,
        return_item_seq_id -> Nullable<Varchar>,
        rejection_id -> Nullable<Varchar>,
        received_by_user_login_id -> Nullable<Varchar>,
        datetime_received -> Nullable<Timestamptz>,
        item_description -> Nullable<Varchar>,
        quantity_accepted -> Nullable<Numeric>,
        quantity_rejected -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_receipt_role (receipt_id, party_id, role_type_id) {
        receipt_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_route_segment (shipment_id, shipment_route_segment_id) {
        shipment_id -> Varchar,
        shipment_route_segment_id -> Varchar,
        delivery_id -> Nullable<Varchar>,
        origin_facility_id -> Nullable<Varchar>,
        dest_facility_id -> Nullable<Varchar>,
        origin_contact_mech_id -> Nullable<Varchar>,
        origin_telecom_number_id -> Nullable<Varchar>,
        dest_contact_mech_id -> Nullable<Varchar>,
        dest_telecom_number_id -> Nullable<Varchar>,
        carrier_party_id -> Nullable<Varchar>,
        shipment_method_type_id -> Nullable<Varchar>,
        carrier_service_status_id -> Nullable<Varchar>,
        carrier_delivery_zone -> Nullable<Varchar>,
        carrier_restriction_codes -> Nullable<Varchar>,
        carrier_restriction_desc -> Nullable<Text>,
        billing_weight -> Nullable<Numeric>,
        billing_weight_uom_id -> Nullable<Varchar>,
        actual_transport_cost -> Nullable<Numeric>,
        actual_service_cost -> Nullable<Numeric>,
        actual_other_cost -> Nullable<Numeric>,
        actual_cost -> Nullable<Numeric>,
        currency_uom_id -> Nullable<Varchar>,
        actual_start_date -> Nullable<Timestamptz>,
        actual_arrival_date -> Nullable<Timestamptz>,
        estimated_start_date -> Nullable<Timestamptz>,
        estimated_arrival_date -> Nullable<Timestamptz>,
        tracking_id_number -> Nullable<Varchar>,
        tracking_digest -> Nullable<Text>,
        updated_by_user_login_id -> Nullable<Varchar>,
        last_updated_date -> Nullable<Timestamptz>,
        home_delivery_type -> Nullable<Varchar>,
        home_delivery_date -> Nullable<Timestamptz>,
        third_party_account_number -> Nullable<Varchar>,
        third_party_postal_code -> Nullable<Varchar>,
        third_party_country_geo_code -> Nullable<Varchar>,
        ups_high_value_report -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_status (status_id, shipment_id) {
        status_id -> Varchar,
        shipment_id -> Varchar,
        status_date -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_time_estimate (shipment_method_type_id, party_id, role_type_id, geo_id_to, geo_id_from, from_date) {
        shipment_method_type_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        geo_id_to -> Varchar,
        geo_id_from -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        lead_time -> Nullable<Numeric>,
        lead_time_uom_id -> Nullable<Varchar>,
        sequence_number -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_type (shipment_type_id) {
        shipment_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipment_type_attr (shipment_type_id, attr_name) {
        shipment_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shipping_document (document_id) {
        document_id -> Varchar,
        shipment_id -> Nullable<Varchar>,
        shipment_item_seq_id -> Nullable<Varchar>,
        shipment_package_seq_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shopping_list (shopping_list_id) {
        shopping_list_id -> Varchar,
        shopping_list_type_id -> Nullable<Varchar>,
        parent_shopping_list_id -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        visitor_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        list_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        is_public -> Nullable<Bpchar>,
        is_active -> Nullable<Bpchar>,
        currency_uom -> Nullable<Varchar>,
        shipment_method_type_id -> Nullable<Varchar>,
        carrier_party_id -> Nullable<Varchar>,
        carrier_role_type_id -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        payment_method_id -> Nullable<Varchar>,
        recurrence_info_id -> Nullable<Varchar>,
        last_ordered_date -> Nullable<Timestamptz>,
        last_admin_modified -> Nullable<Timestamptz>,
        product_promo_code_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shopping_list_item (shopping_list_id, shopping_list_item_seq_id) {
        shopping_list_id -> Varchar,
        shopping_list_item_seq_id -> Varchar,
        product_id -> Nullable<Varchar>,
        quantity -> Nullable<Numeric>,
        modified_price -> Nullable<Numeric>,
        reserv_start -> Nullable<Timestamptz>,
        reserv_length -> Nullable<Numeric>,
        reserv_persons -> Nullable<Numeric>,
        quantity_purchased -> Nullable<Numeric>,
        config_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shopping_list_item_survey (shopping_list_id, shopping_list_item_seq_id, survey_response_id) {
        shopping_list_id -> Varchar,
        shopping_list_item_seq_id -> Varchar,
        survey_response_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shopping_list_type (shopping_list_type_id) {
        shopping_list_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    shopping_list_work_effort (shopping_list_id, work_effort_id) {
        shopping_list_id -> Varchar,
        work_effort_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    skill_type (skill_type_id) {
        skill_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    standard_language (standard_language_id) {
        standard_language_id -> Varchar,
        lang_code3t -> Nullable<Varchar>,
        lang_code3b -> Nullable<Varchar>,
        lang_code2 -> Nullable<Varchar>,
        lang_name -> Nullable<Varchar>,
        lang_family -> Nullable<Varchar>,
        lang_charset -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    status_item (status_id) {
        status_id -> Varchar,
        status_type_id -> Nullable<Varchar>,
        status_code -> Nullable<Varchar>,
        sequence_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    status_type (status_type_id) {
        status_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    status_valid_change (status_id, status_id_to) {
        status_id -> Varchar,
        status_id_to -> Varchar,
        condition_expression -> Nullable<Varchar>,
        transition_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription (subscription_id) {
        subscription_id -> Varchar,
        description -> Nullable<Varchar>,
        subscription_resource_id -> Nullable<Varchar>,
        communication_event_id -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        originated_from_party_id -> Nullable<Varchar>,
        originated_from_role_type_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
        party_need_id -> Nullable<Varchar>,
        need_type_id -> Nullable<Varchar>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        product_id -> Nullable<Varchar>,
        product_category_id -> Nullable<Varchar>,
        inventory_item_id -> Nullable<Varchar>,
        subscription_type_id -> Nullable<Varchar>,
        external_subscription_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        purchase_from_date -> Nullable<Timestamptz>,
        purchase_thru_date -> Nullable<Timestamptz>,
        max_life_time -> Nullable<Numeric>,
        max_life_time_uom_id -> Nullable<Varchar>,
        available_time -> Nullable<Numeric>,
        available_time_uom_id -> Nullable<Varchar>,
        use_count_limit -> Nullable<Numeric>,
        use_time -> Nullable<Numeric>,
        use_time_uom_id -> Nullable<Varchar>,
        automatic_extend -> Nullable<Bpchar>,
        cancl_autm_ext_time -> Nullable<Numeric>,
        cancl_autm_ext_time_uom_id -> Nullable<Varchar>,
        grace_period_on_expiry -> Nullable<Numeric>,
        grace_period_on_expiry_uom_id -> Nullable<Varchar>,
        expiration_completed_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription_activity (subscription_activity_id) {
        subscription_activity_id -> Varchar,
        comments -> Nullable<Varchar>,
        date_sent -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription_attribute (subscription_id, attr_name) {
        subscription_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription_comm_event (subscription_id, communication_event_id) {
        subscription_id -> Varchar,
        communication_event_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription_fulfillment_piece (subscription_activity_id, subscription_id) {
        subscription_activity_id -> Varchar,
        subscription_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription_resource (subscription_resource_id) {
        subscription_resource_id -> Varchar,
        parent_resource_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        content_id -> Nullable<Varchar>,
        web_site_id -> Nullable<Varchar>,
        service_name_on_expiry -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription_type (subscription_type_id) {
        subscription_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    subscription_type_attr (subscription_type_id, attr_name) {
        subscription_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    supplier_pref_order (supplier_pref_order_id) {
        supplier_pref_order_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    supplier_product (product_id, party_id, currency_uom_id, minimum_order_quantity, available_from_date) {
        product_id -> Varchar,
        party_id -> Varchar,
        available_from_date -> Timestamptz,
        available_thru_date -> Nullable<Timestamptz>,
        supplier_pref_order_id -> Nullable<Varchar>,
        supplier_rating_type_id -> Nullable<Varchar>,
        standard_lead_time_days -> Nullable<Numeric>,
        minimum_order_quantity -> Numeric,
        order_qty_increments -> Nullable<Numeric>,
        units_included -> Nullable<Numeric>,
        quantity_uom_id -> Nullable<Varchar>,
        agreement_id -> Nullable<Varchar>,
        agreement_item_seq_id -> Nullable<Varchar>,
        last_price -> Nullable<Numeric>,
        shipping_price -> Nullable<Numeric>,
        currency_uom_id -> Varchar,
        supplier_product_name -> Nullable<Varchar>,
        supplier_product_id -> Nullable<Varchar>,
        can_drop_ship -> Nullable<Bpchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    supplier_product_feature (party_id, product_feature_id) {
        party_id -> Varchar,
        product_feature_id -> Varchar,
        description -> Nullable<Varchar>,
        uom_id -> Nullable<Varchar>,
        id_code -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    supplier_rating_type (supplier_rating_type_id) {
        supplier_rating_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey (survey_id) {
        survey_id -> Varchar,
        survey_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        submit_caption -> Nullable<Varchar>,
        response_service -> Nullable<Varchar>,
        is_anonymous -> Nullable<Bpchar>,
        allow_multiple -> Nullable<Bpchar>,
        allow_update -> Nullable<Bpchar>,
        acro_form_content_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_appl_type (survey_appl_type_id) {
        survey_appl_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_multi_resp (survey_id, survey_multi_resp_id) {
        survey_id -> Varchar,
        survey_multi_resp_id -> Varchar,
        multi_resp_title -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_multi_resp_column (survey_id, survey_multi_resp_id, survey_multi_resp_col_id) {
        survey_id -> Varchar,
        survey_multi_resp_id -> Varchar,
        survey_multi_resp_col_id -> Varchar,
        column_title -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_page (survey_id, survey_page_seq_id) {
        survey_id -> Varchar,
        survey_page_seq_id -> Varchar,
        page_name -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_question (survey_question_id) {
        survey_question_id -> Varchar,
        survey_question_category_id -> Nullable<Varchar>,
        survey_question_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        question -> Nullable<Text>,
        hint -> Nullable<Text>,
        enum_type_id -> Nullable<Varchar>,
        geo_id -> Nullable<Varchar>,
        format_string -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_question_appl (survey_id, survey_question_id, from_date) {
        survey_id -> Varchar,
        survey_question_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        survey_page_seq_id -> Nullable<Varchar>,
        survey_multi_resp_id -> Nullable<Varchar>,
        survey_multi_resp_col_id -> Nullable<Varchar>,
        required_field -> Nullable<Bpchar>,
        sequence_num -> Nullable<Numeric>,
        external_field_ref -> Nullable<Varchar>,
        with_survey_question_id -> Nullable<Varchar>,
        with_survey_option_seq_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_question_category (survey_question_category_id) {
        survey_question_category_id -> Varchar,
        parent_category_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_question_option (survey_question_id, survey_option_seq_id) {
        survey_question_id -> Varchar,
        survey_option_seq_id -> Varchar,
        description -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        amount_base -> Nullable<Numeric>,
        amount_base_uom_id -> Nullable<Varchar>,
        weight_factor -> Nullable<Float8>,
        duration -> Nullable<Numeric>,
        duration_uom_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_question_type (survey_question_type_id) {
        survey_question_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_response (survey_response_id) {
        survey_response_id -> Varchar,
        survey_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        response_date -> Nullable<Timestamptz>,
        last_modified_date -> Nullable<Timestamptz>,
        reference_id -> Nullable<Varchar>,
        general_feedback -> Nullable<Text>,
        order_id -> Nullable<Varchar>,
        order_item_seq_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_response_answer (survey_response_id, survey_question_id, survey_multi_resp_col_id) {
        survey_response_id -> Varchar,
        survey_question_id -> Varchar,
        survey_multi_resp_col_id -> Varchar,
        survey_multi_resp_id -> Nullable<Varchar>,
        boolean_response -> Nullable<Bpchar>,
        currency_response -> Nullable<Numeric>,
        float_response -> Nullable<Float8>,
        numeric_response -> Nullable<Numeric>,
        text_response -> Nullable<Text>,
        survey_option_seq_id -> Nullable<Varchar>,
        content_id -> Nullable<Varchar>,
        answered_date -> Nullable<Timestamptz>,
        amount_base -> Nullable<Numeric>,
        amount_base_uom_id -> Nullable<Varchar>,
        weight_factor -> Nullable<Float8>,
        duration -> Nullable<Numeric>,
        duration_uom_id -> Nullable<Varchar>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    survey_trigger (survey_id, survey_appl_type_id, from_date) {
        survey_id -> Varchar,
        survey_appl_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    system_property (system_resource_id, system_property_id) {
        system_resource_id -> Varchar,
        system_property_id -> Varchar,
        system_property_value -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tarpitted_login_view (view_name_id, user_login_id) {
        view_name_id -> Varchar,
        user_login_id -> Varchar,
        tarpit_release_date_time -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tax_authority (tax_auth_geo_id, tax_auth_party_id) {
        tax_auth_geo_id -> Varchar,
        tax_auth_party_id -> Varchar,
        require_tax_id_for_exemption -> Nullable<Bpchar>,
        tax_id_format_pattern -> Nullable<Varchar>,
        include_tax_in_price -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tax_authority_assoc (tax_auth_geo_id, tax_auth_party_id, to_tax_auth_geo_id, to_tax_auth_party_id, from_date) {
        tax_auth_geo_id -> Varchar,
        tax_auth_party_id -> Varchar,
        to_tax_auth_geo_id -> Varchar,
        to_tax_auth_party_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        tax_authority_assoc_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tax_authority_assoc_type (tax_authority_assoc_type_id) {
        tax_authority_assoc_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tax_authority_category (tax_auth_geo_id, tax_auth_party_id, product_category_id) {
        tax_auth_geo_id -> Varchar,
        tax_auth_party_id -> Varchar,
        product_category_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tax_authority_gl_account (tax_auth_geo_id, tax_auth_party_id, organization_party_id) {
        tax_auth_geo_id -> Varchar,
        tax_auth_party_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tax_authority_rate_product (tax_authority_rate_seq_id) {
        tax_authority_rate_seq_id -> Varchar,
        tax_auth_geo_id -> Nullable<Varchar>,
        tax_auth_party_id -> Nullable<Varchar>,
        tax_authority_rate_type_id -> Nullable<Varchar>,
        product_store_id -> Nullable<Varchar>,
        product_category_id -> Nullable<Varchar>,
        title_transfer_enum_id -> Nullable<Varchar>,
        min_item_price -> Nullable<Numeric>,
        min_purchase -> Nullable<Numeric>,
        tax_shipping -> Nullable<Bpchar>,
        tax_percentage -> Nullable<Numeric>,
        tax_promotions -> Nullable<Bpchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        is_tax_in_shipping_price -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tax_authority_rate_type (tax_authority_rate_type_id) {
        tax_authority_rate_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tech_data_calendar (calendar_id) {
        calendar_id -> Varchar,
        description -> Nullable<Varchar>,
        calendar_week_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tech_data_calendar_exc_day (calendar_id, exception_date_start_time) {
        calendar_id -> Varchar,
        exception_date_start_time -> Timestamptz,
        exception_capacity -> Nullable<Numeric>,
        used_capacity -> Nullable<Numeric>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tech_data_calendar_exc_week (calendar_id, exception_date_start) {
        calendar_id -> Varchar,
        exception_date_start -> Date,
        calendar_week_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tech_data_calendar_week (calendar_week_id) {
        calendar_week_id -> Varchar,
        description -> Nullable<Varchar>,
        monday_start_time -> Nullable<Time>,
        monday_capacity -> Nullable<Float8>,
        tuesday_start_time -> Nullable<Time>,
        tuesday_capacity -> Nullable<Float8>,
        wednesday_start_time -> Nullable<Time>,
        wednesday_capacity -> Nullable<Float8>,
        thursday_start_time -> Nullable<Time>,
        thursday_capacity -> Nullable<Float8>,
        friday_start_time -> Nullable<Time>,
        friday_capacity -> Nullable<Float8>,
        saturday_start_time -> Nullable<Time>,
        saturday_capacity -> Nullable<Float8>,
        sunday_start_time -> Nullable<Time>,
        sunday_capacity -> Nullable<Float8>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    telecom_gateway_config (telecom_gateway_config_id) {
        telecom_gateway_config_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    telecom_method_type (telecom_method_type_id) {
        telecom_method_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    telecom_number (contact_mech_id) {
        contact_mech_id -> Varchar,
        country_code -> Nullable<Varchar>,
        area_code -> Nullable<Varchar>,
        contact_number -> Nullable<Varchar>,
        ask_for_name -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    temporal_expression (temp_expr_id) {
        temp_expr_id -> Varchar,
        temp_expr_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        date1 -> Nullable<Timestamptz>,
        date2 -> Nullable<Timestamptz>,
        integer1 -> Nullable<Numeric>,
        integer2 -> Nullable<Numeric>,
        string1 -> Nullable<Varchar>,
        string2 -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    temporal_expression_assoc (from_temp_expr_id, to_temp_expr_id) {
        from_temp_expr_id -> Varchar,
        to_temp_expr_id -> Varchar,
        expr_assoc_type -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    term_type (term_type_id) {
        term_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    term_type_attr (term_type_id, attr_name) {
        term_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    termination_reason (termination_reason_id) {
        termination_reason_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    termination_type (termination_type_id) {
        termination_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    test_field_type (test_field_type_id) {
        test_field_type_id -> Varchar,
        blob_field -> Nullable<Bytea>,
        byte_array_field -> Nullable<Bytea>,
        object_field -> Nullable<Bytea>,
        date_field -> Nullable<Date>,
        time_field -> Nullable<Time>,
        date_time_field -> Nullable<Timestamptz>,
        fixed_point_field -> Nullable<Numeric>,
        floating_point_field -> Nullable<Float8>,
        numeric_field -> Nullable<Numeric>,
        clob_field -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing (testing_id) {
        testing_id -> Varchar,
        testing_type_id -> Nullable<Varchar>,
        testing_name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        testing_size -> Nullable<Numeric>,
        testing_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_crypto (testing_crypto_id) {
        testing_crypto_id -> Varchar,
        testing_crypto_type_id -> Nullable<Varchar>,
        unencrypted_value -> Nullable<Varchar>,
        encrypted_value -> Nullable<Varchar>,
        salted_encrypted_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_item (testing_id, testing_seq_id) {
        testing_id -> Varchar,
        testing_seq_id -> Varchar,
        testing_history -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_node (testing_node_id) {
        testing_node_id -> Varchar,
        primary_parent_node_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_node_member (testing_node_id, testing_id, from_date) {
        testing_node_id -> Varchar,
        testing_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        extend_from_date -> Nullable<Timestamptz>,
        extend_thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_remove_all (testing_remove_all_id) {
        testing_remove_all_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_status (testing_status_id) {
        testing_status_id -> Varchar,
        testing_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        status_date -> Nullable<Timestamptz>,
        change_by_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_subtype (testing_type_id) {
        testing_type_id -> Varchar,
        subtype_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    testing_type (testing_type_id) {
        testing_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    third_party_login (product_store_id, login_meth_type_id, login_provider_id, from_date) {
        product_store_id -> Varchar,
        login_meth_type_id -> Varchar,
        login_provider_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    time_entry (time_entry_id) {
        time_entry_id -> Varchar,
        party_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        rate_type_id -> Nullable<Varchar>,
        work_effort_id -> Nullable<Varchar>,
        timesheet_id -> Nullable<Varchar>,
        invoice_id -> Nullable<Varchar>,
        invoice_item_seq_id -> Nullable<Varchar>,
        hours -> Nullable<Float8>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        plan_hours -> Nullable<Float8>,
    }
}

table! {
    timesheet (timesheet_id) {
        timesheet_id -> Varchar,
        party_id -> Nullable<Varchar>,
        client_party_id -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Varchar>,
        approved_by_user_login_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    timesheet_role (timesheet_id, party_id, role_type_id) {
        timesheet_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tracking_code (tracking_code_id) {
        tracking_code_id -> Varchar,
        tracking_code_type_id -> Nullable<Varchar>,
        marketing_campaign_id -> Nullable<Varchar>,
        redirect_url -> Nullable<Varchar>,
        override_logo -> Nullable<Varchar>,
        override_css -> Nullable<Varchar>,
        prod_catalog_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        trackable_lifetime -> Nullable<Numeric>,
        billable_lifetime -> Nullable<Numeric>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        group_id -> Nullable<Varchar>,
        subgroup_id -> Nullable<Varchar>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tracking_code_order (order_id, tracking_code_type_id) {
        order_id -> Varchar,
        tracking_code_type_id -> Varchar,
        tracking_code_id -> Nullable<Varchar>,
        is_billable -> Nullable<Bpchar>,
        site_id -> Nullable<Varchar>,
        has_exported -> Nullable<Bpchar>,
        affiliate_referred_time_stamp -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tracking_code_order_return (return_id, order_id, tracking_code_type_id) {
        return_id -> Varchar,
        order_id -> Varchar,
        order_item_seq_id -> Nullable<Varchar>,
        tracking_code_type_id -> Varchar,
        tracking_code_id -> Nullable<Varchar>,
        is_billable -> Nullable<Bpchar>,
        site_id -> Nullable<Varchar>,
        has_exported -> Nullable<Bpchar>,
        affiliate_referred_time_stamp -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tracking_code_type (tracking_code_type_id) {
        tracking_code_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    tracking_code_visit (tracking_code_id, visit_id, from_date) {
        tracking_code_id -> Varchar,
        visit_id -> Varchar,
        from_date -> Timestamptz,
        source_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    training_class_type (training_class_type_id) {
        training_class_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    training_request (training_request_id) {
        training_request_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    unemployment_claim (unemployment_claim_id) {
        unemployment_claim_id -> Varchar,
        unemployment_claim_date -> Nullable<Timestamptz>,
        description -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        party_id_from -> Nullable<Varchar>,
        party_id_to -> Nullable<Varchar>,
        role_type_id_from -> Nullable<Varchar>,
        role_type_id_to -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    uom (uom_id) {
        uom_id -> Varchar,
        uom_type_id -> Nullable<Varchar>,
        abbreviation -> Nullable<Varchar>,
        numeric_code -> Nullable<Numeric>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    uom_conversion (uom_id, uom_id_to) {
        uom_id -> Varchar,
        uom_id_to -> Varchar,
        conversion_factor -> Nullable<Float8>,
        custom_method_id -> Nullable<Varchar>,
        decimal_scale -> Nullable<Numeric>,
        rounding_mode -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    uom_conversion_dated (uom_id, uom_id_to, from_date) {
        uom_id -> Varchar,
        uom_id_to -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        conversion_factor -> Nullable<Float8>,
        custom_method_id -> Nullable<Varchar>,
        decimal_scale -> Nullable<Numeric>,
        rounding_mode -> Nullable<Varchar>,
        purpose_enum_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    uom_group (uom_group_id, uom_id) {
        uom_group_id -> Varchar,
        uom_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    uom_type (uom_type_id) {
        uom_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_agent (user_agent_id) {
        user_agent_id -> Varchar,
        browser_type_id -> Nullable<Varchar>,
        platform_type_id -> Nullable<Varchar>,
        protocol_type_id -> Nullable<Varchar>,
        user_agent_type_id -> Nullable<Varchar>,
        user_agent_method_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_agent_method_type (user_agent_method_type_id) {
        user_agent_method_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_agent_type (user_agent_type_id) {
        user_agent_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_login (user_login_id) {
        user_login_id -> Varchar,
        current_password -> Nullable<Varchar>,
        password_hint -> Nullable<Varchar>,
        is_system -> Nullable<Bpchar>,
        enabled -> Nullable<Bpchar>,
        has_logged_out -> Nullable<Bpchar>,
        require_password_change -> Nullable<Bpchar>,
        last_currency_uom -> Nullable<Varchar>,
        last_locale -> Nullable<Varchar>,
        last_time_zone -> Nullable<Varchar>,
        disabled_date_time -> Nullable<Timestamptz>,
        successive_failed_logins -> Nullable<Numeric>,
        external_auth_id -> Nullable<Varchar>,
        user_ldap_dn -> Nullable<Varchar>,
        disabled_by -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        party_id -> Nullable<Varchar>,
    }
}

table! {
    user_login_history (user_login_id, from_date) {
        user_login_id -> Varchar,
        visit_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        password_used -> Nullable<Varchar>,
        successful_login -> Nullable<Bpchar>,
        origin_user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        party_id -> Nullable<Varchar>,
    }
}

table! {
    user_login_password_history (user_login_id, from_date) {
        user_login_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        current_password -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_login_security_group (user_login_id, group_id, from_date) {
        user_login_id -> Varchar,
        group_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_login_session (user_login_id) {
        user_login_id -> Varchar,
        saved_date -> Nullable<Timestamptz>,
        session_data -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_pref_group_type (user_pref_group_type_id) {
        user_pref_group_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    user_preference (user_login_id, user_pref_type_id) {
        user_login_id -> Varchar,
        user_pref_type_id -> Varchar,
        user_pref_group_type_id -> Nullable<Varchar>,
        user_pref_value -> Nullable<Varchar>,
        user_pref_data_type -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    valid_contact_mech_role (role_type_id, contact_mech_type_id) {
        role_type_id -> Varchar,
        contact_mech_type_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    valid_responsibility (empl_position_type_id, responsibility_type_id, from_date) {
        empl_position_type_id -> Varchar,
        responsibility_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    value_link_key (merchant_id) {
        merchant_id -> Varchar,
        public_key -> Nullable<Text>,
        private_key -> Nullable<Text>,
        exchange_key -> Nullable<Text>,
        working_key -> Nullable<Text>,
        working_key_index -> Nullable<Numeric>,
        last_working_key -> Nullable<Text>,
        created_date -> Nullable<Timestamptz>,
        created_by_terminal -> Nullable<Varchar>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_terminal -> Nullable<Varchar>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    variance_reason (variance_reason_id) {
        variance_reason_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    variance_reason_gl_account (variance_reason_id, organization_party_id) {
        variance_reason_id -> Varchar,
        organization_party_id -> Varchar,
        gl_account_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    vendor (party_id) {
        party_id -> Varchar,
        manifest_company_name -> Nullable<Varchar>,
        manifest_company_title -> Nullable<Varchar>,
        manifest_logo_url -> Nullable<Varchar>,
        manifest_policies -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    vendor_product (product_id, vendor_party_id, product_store_group_id) {
        product_id -> Varchar,
        vendor_party_id -> Varchar,
        product_store_group_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    video_data_resource (data_resource_id) {
        data_resource_id -> Varchar,
        video_data -> Nullable<Bytea>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    visit (visit_id) {
        visit_id -> Varchar,
        visitor_id -> Nullable<Varchar>,
        user_login_id -> Nullable<Varchar>,
        user_created -> Nullable<Bpchar>,
        session_id -> Nullable<Varchar>,
        server_ip_address -> Nullable<Varchar>,
        server_host_name -> Nullable<Varchar>,
        webapp_name -> Nullable<Varchar>,
        initial_locale -> Nullable<Varchar>,
        initial_request -> Nullable<Varchar>,
        initial_referrer -> Nullable<Varchar>,
        initial_user_agent -> Nullable<Varchar>,
        user_agent_id -> Nullable<Varchar>,
        client_ip_address -> Nullable<Varchar>,
        client_host_name -> Nullable<Varchar>,
        client_user -> Nullable<Varchar>,
        client_ip_isp_name -> Nullable<Varchar>,
        client_ip_postal_code -> Nullable<Varchar>,
        cookie -> Nullable<Varchar>,
        from_date -> Nullable<Timestamptz>,
        thru_date -> Nullable<Timestamptz>,
        client_ip_state_prov_geo_id -> Nullable<Varchar>,
        client_ip_country_geo_id -> Nullable<Varchar>,
        contact_mech_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        role_type_id -> Nullable<Varchar>,
    }
}

table! {
    visitor (visitor_id) {
        visitor_id -> Varchar,
        user_login_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        party_id -> Nullable<Varchar>,
    }
}

table! {
    visual_theme (visual_theme_id) {
        visual_theme_id -> Varchar,
        visual_theme_set_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    visual_theme_resource (visual_theme_id, resource_type_enum_id, sequence_id) {
        visual_theme_id -> Varchar,
        resource_type_enum_id -> Varchar,
        sequence_id -> Varchar,
        resource_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    visual_theme_set (visual_theme_set_id) {
        visual_theme_set_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_analytics_config (web_site_id, web_analytics_type_id) {
        web_site_id -> Varchar,
        web_analytics_type_id -> Varchar,
        web_analytics_code -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_analytics_type (web_analytics_type_id) {
        web_analytics_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_page (web_page_id) {
        web_page_id -> Varchar,
        page_name -> Nullable<Varchar>,
        web_site_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        content_id -> Nullable<Varchar>,
    }
}

table! {
    web_preference_type (web_preference_type_id) {
        web_preference_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_site (web_site_id) {
        web_site_id -> Varchar,
        site_name -> Nullable<Varchar>,
        http_host -> Nullable<Varchar>,
        http_port -> Nullable<Varchar>,
        https_host -> Nullable<Varchar>,
        https_port -> Nullable<Varchar>,
        enable_https -> Nullable<Bpchar>,
        webapp_path -> Nullable<Varchar>,
        standard_content_prefix -> Nullable<Varchar>,
        secure_content_prefix -> Nullable<Varchar>,
        cookie_domain -> Nullable<Varchar>,
        visual_theme_set_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        product_store_id -> Nullable<Varchar>,
        allow_product_store_change -> Nullable<Bpchar>,
        hosted_path_alias -> Nullable<Varchar>,
        is_default -> Nullable<Bpchar>,
        display_maintenance_page -> Nullable<Bpchar>,
    }
}

table! {
    web_site_contact_list (web_site_id, contact_list_id, from_date) {
        web_site_id -> Varchar,
        contact_list_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_site_content (web_site_id, content_id, web_site_content_type_id, from_date) {
        web_site_id -> Varchar,
        content_id -> Varchar,
        web_site_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_site_content_type (web_site_content_type_id) {
        web_site_content_type_id -> Varchar,
        description -> Nullable<Varchar>,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_site_path_alias (web_site_id, path_alias, from_date) {
        web_site_id -> Varchar,
        path_alias -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        alias_to -> Nullable<Varchar>,
        content_id -> Nullable<Varchar>,
        map_key -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_site_publish_point (content_id) {
        content_id -> Varchar,
        template_title -> Nullable<Varchar>,
        style_sheet_file -> Nullable<Varchar>,
        logo -> Nullable<Varchar>,
        medallion_logo -> Nullable<Varchar>,
        line_logo -> Nullable<Varchar>,
        left_bar_id -> Nullable<Varchar>,
        right_bar_id -> Nullable<Varchar>,
        content_dept -> Nullable<Varchar>,
        about_content_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_site_role (party_id, role_type_id, web_site_id, from_date) {
        party_id -> Varchar,
        role_type_id -> Varchar,
        web_site_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    web_user_preference (user_login_id, party_id, visit_id, web_preference_type_id) {
        user_login_id -> Varchar,
        party_id -> Varchar,
        visit_id -> Varchar,
        web_preference_type_id -> Varchar,
        web_preference_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort (work_effort_id) {
        work_effort_id -> Varchar,
        work_effort_type_id -> Nullable<Varchar>,
        current_status_id -> Nullable<Varchar>,
        last_status_update -> Nullable<Timestamptz>,
        work_effort_purpose_type_id -> Nullable<Varchar>,
        work_effort_parent_id -> Nullable<Varchar>,
        scope_enum_id -> Nullable<Varchar>,
        priority -> Nullable<Numeric>,
        percent_complete -> Nullable<Numeric>,
        work_effort_name -> Nullable<Varchar>,
        show_as_enum_id -> Nullable<Varchar>,
        send_notification_email -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        location_desc -> Nullable<Varchar>,
        estimated_start_date -> Nullable<Timestamptz>,
        estimated_completion_date -> Nullable<Timestamptz>,
        actual_start_date -> Nullable<Timestamptz>,
        actual_completion_date -> Nullable<Timestamptz>,
        estimated_milli_seconds -> Nullable<Float8>,
        estimated_setup_millis -> Nullable<Float8>,
        estimate_calc_method -> Nullable<Varchar>,
        actual_milli_seconds -> Nullable<Float8>,
        actual_setup_millis -> Nullable<Float8>,
        total_milli_seconds_allowed -> Nullable<Float8>,
        total_money_allowed -> Nullable<Numeric>,
        money_uom_id -> Nullable<Varchar>,
        special_terms -> Nullable<Varchar>,
        time_transparency -> Nullable<Numeric>,
        universal_id -> Nullable<Varchar>,
        source_reference_id -> Nullable<Varchar>,
        fixed_asset_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        info_url -> Nullable<Varchar>,
        recurrence_info_id -> Nullable<Varchar>,
        temp_expr_id -> Nullable<Varchar>,
        runtime_data_id -> Nullable<Varchar>,
        note_id -> Nullable<Varchar>,
        service_loader_name -> Nullable<Varchar>,
        quantity_to_produce -> Nullable<Numeric>,
        quantity_produced -> Nullable<Numeric>,
        quantity_rejected -> Nullable<Numeric>,
        reserv_persons -> Nullable<Numeric>,
        reserv2nd_p_p_perc -> Nullable<Numeric>,
        reserv_nth_p_p_perc -> Nullable<Numeric>,
        accommodation_map_id -> Nullable<Varchar>,
        accommodation_spot_id -> Nullable<Varchar>,
        revision_number -> Nullable<Numeric>,
        created_date -> Nullable<Timestamptz>,
        created_by_user_login -> Nullable<Varchar>,
        last_modified_date -> Nullable<Timestamptz>,
        last_modified_by_user_login -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
        sequence_num -> Nullable<Numeric>,
    }
}

table! {
    work_effort_assoc (work_effort_id_from, work_effort_id_to, work_effort_assoc_type_id, from_date) {
        work_effort_id_from -> Varchar,
        work_effort_id_to -> Varchar,
        work_effort_assoc_type_id -> Varchar,
        sequence_num -> Nullable<Numeric>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_assoc_attribute (work_effort_id_from, work_effort_id_to, work_effort_assoc_type_id, attr_name) {
        work_effort_id_from -> Varchar,
        work_effort_id_to -> Varchar,
        work_effort_assoc_type_id -> Varchar,
        from_date -> Nullable<Timestamptz>,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_assoc_type (work_effort_assoc_type_id) {
        work_effort_assoc_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_assoc_type_attr (work_effort_assoc_type_id, attr_name) {
        work_effort_assoc_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_attribute (work_effort_id, attr_name) {
        work_effort_id -> Varchar,
        attr_name -> Varchar,
        attr_value -> Nullable<Varchar>,
        attr_description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_billing (work_effort_id, invoice_id, invoice_item_seq_id) {
        work_effort_id -> Varchar,
        invoice_id -> Varchar,
        invoice_item_seq_id -> Varchar,
        percentage -> Nullable<Float8>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_contact_mech_new (work_effort_id, contact_mech_id, from_date) {
        work_effort_id -> Varchar,
        contact_mech_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_content (work_effort_id, content_id, work_effort_content_type_id, from_date) {
        work_effort_id -> Varchar,
        content_id -> Varchar,
        work_effort_content_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_content_type (work_effort_content_type_id) {
        work_effort_content_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_cost_calc (work_effort_id, cost_component_type_id, from_date) {
        work_effort_id -> Varchar,
        cost_component_type_id -> Varchar,
        cost_component_calc_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_deliverable_prod (work_effort_id, deliverable_id) {
        work_effort_id -> Varchar,
        deliverable_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_event_reminder (work_effort_id, sequence_id) {
        work_effort_id -> Varchar,
        sequence_id -> Varchar,
        contact_mech_id -> Nullable<Varchar>,
        party_id -> Nullable<Varchar>,
        reminder_date_time -> Nullable<Timestamptz>,
        repeat_count -> Nullable<Numeric>,
        repeat_interval -> Nullable<Numeric>,
        current_count -> Nullable<Numeric>,
        reminder_offset -> Nullable<Numeric>,
        locale_id -> Nullable<Varchar>,
        time_zone_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_fixed_asset_assign (work_effort_id, fixed_asset_id, from_date) {
        work_effort_id -> Varchar,
        fixed_asset_id -> Varchar,
        status_id -> Nullable<Varchar>,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        availability_status_id -> Nullable<Varchar>,
        allocated_cost -> Nullable<Numeric>,
        comments -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_fixed_asset_std (work_effort_id, fixed_asset_type_id) {
        work_effort_id -> Varchar,
        fixed_asset_type_id -> Varchar,
        estimated_quantity -> Nullable<Float8>,
        estimated_duration -> Nullable<Float8>,
        estimated_cost -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_good_standard (work_effort_id, product_id, work_effort_good_std_type_id, from_date) {
        work_effort_id -> Varchar,
        product_id -> Varchar,
        work_effort_good_std_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        status_id -> Nullable<Varchar>,
        estimated_quantity -> Nullable<Float8>,
        estimated_cost -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_good_standard_type (work_effort_good_std_type_id) {
        work_effort_good_std_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_ical_data (work_effort_id) {
        work_effort_id -> Varchar,
        ical_data -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_inventory_assign (work_effort_id, inventory_item_id) {
        work_effort_id -> Varchar,
        inventory_item_id -> Varchar,
        status_id -> Nullable<Varchar>,
        quantity -> Nullable<Float8>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_inventory_produced (work_effort_id, inventory_item_id) {
        work_effort_id -> Varchar,
        inventory_item_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_keyword (work_effort_id, keyword) {
        work_effort_id -> Varchar,
        keyword -> Varchar,
        relevancy_weight -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_note (work_effort_id, note_id) {
        work_effort_id -> Varchar,
        note_id -> Varchar,
        internal_note -> Nullable<Bpchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_party_assignment (work_effort_id, party_id, role_type_id, from_date) {
        work_effort_id -> Varchar,
        party_id -> Varchar,
        role_type_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        assigned_by_user_login_id -> Nullable<Varchar>,
        status_id -> Nullable<Varchar>,
        status_date_time -> Nullable<Timestamptz>,
        expectation_enum_id -> Nullable<Varchar>,
        delegate_reason_enum_id -> Nullable<Varchar>,
        facility_id -> Nullable<Varchar>,
        comments -> Nullable<Varchar>,
        must_rsvp -> Nullable<Bpchar>,
        availability_status_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_purpose_type (work_effort_purpose_type_id) {
        work_effort_purpose_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_review (work_effort_id, user_login_id, review_date) {
        work_effort_id -> Varchar,
        user_login_id -> Varchar,
        review_date -> Timestamptz,
        status_id -> Nullable<Varchar>,
        posted_anonymous -> Nullable<Bpchar>,
        rating -> Nullable<Float8>,
        review_text -> Nullable<Text>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_search_constraint (work_effort_search_result_id, constraint_seq_id) {
        work_effort_search_result_id -> Varchar,
        constraint_seq_id -> Varchar,
        constraint_name -> Nullable<Varchar>,
        info_string -> Nullable<Varchar>,
        include_sub_work_efforts -> Nullable<Bpchar>,
        is_and -> Nullable<Bpchar>,
        any_prefix -> Nullable<Bpchar>,
        any_suffix -> Nullable<Bpchar>,
        remove_stems -> Nullable<Bpchar>,
        low_value -> Nullable<Varchar>,
        high_value -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_search_result (work_effort_search_result_id) {
        work_effort_search_result_id -> Varchar,
        visit_id -> Nullable<Varchar>,
        order_by_name -> Nullable<Varchar>,
        is_ascending -> Nullable<Bpchar>,
        num_results -> Nullable<Numeric>,
        seconds_total -> Nullable<Float8>,
        search_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_skill_standard (work_effort_id, skill_type_id) {
        work_effort_id -> Varchar,
        skill_type_id -> Varchar,
        estimated_num_people -> Nullable<Float8>,
        estimated_duration -> Nullable<Float8>,
        estimated_cost -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_status (work_effort_id, status_id, status_datetime) {
        work_effort_id -> Varchar,
        status_id -> Varchar,
        status_datetime -> Timestamptz,
        set_by_user_login -> Nullable<Varchar>,
        reason -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_survey_appl (work_effort_id, survey_id, from_date) {
        work_effort_id -> Varchar,
        survey_id -> Varchar,
        from_date -> Timestamptz,
        thru_date -> Nullable<Timestamptz>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_trans_box (process_work_effort_id, to_activity_id, transition_id) {
        process_work_effort_id -> Varchar,
        to_activity_id -> Varchar,
        transition_id -> Varchar,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_type (work_effort_type_id) {
        work_effort_type_id -> Varchar,
        parent_type_id -> Nullable<Varchar>,
        has_table -> Nullable<Bpchar>,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_effort_type_attr (work_effort_type_id, attr_name) {
        work_effort_type_id -> Varchar,
        attr_name -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_order_item_fulfillment (work_effort_id, order_id, order_item_seq_id) {
        work_effort_id -> Varchar,
        order_id -> Varchar,
        order_item_seq_id -> Varchar,
        ship_group_seq_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_req_fulf_type (work_req_fulf_type_id) {
        work_req_fulf_type_id -> Varchar,
        description -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    work_requirement_fulfillment (requirement_id, work_effort_id) {
        requirement_id -> Varchar,
        work_effort_id -> Varchar,
        work_req_fulf_type_id -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    x509_issuer_provision (cert_provision_id) {
        cert_provision_id -> Varchar,
        common_name -> Nullable<Varchar>,
        organizational_unit -> Nullable<Varchar>,
        organization_name -> Nullable<Varchar>,
        city_locality -> Nullable<Varchar>,
        state_province -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        serial_number -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    zip_sales_rule_lookup (state_code, city, county, from_date) {
        state_code -> Varchar,
        city -> Varchar,
        county -> Varchar,
        from_date -> Timestamptz,
        id_code -> Nullable<Varchar>,
        taxable -> Nullable<Varchar>,
        ship_cond -> Nullable<Varchar>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

table! {
    zip_sales_tax_lookup (zip_code, state_code, city, county, from_date) {
        zip_code -> Varchar,
        state_code -> Varchar,
        city -> Varchar,
        county -> Varchar,
        from_date -> Timestamptz,
        county_fips -> Nullable<Varchar>,
        county_default -> Nullable<Bpchar>,
        general_default -> Nullable<Bpchar>,
        inside_city -> Nullable<Bpchar>,
        geo_code -> Nullable<Varchar>,
        state_sales_tax -> Nullable<Numeric>,
        city_sales_tax -> Nullable<Numeric>,
        city_local_sales_tax -> Nullable<Numeric>,
        county_sales_tax -> Nullable<Numeric>,
        county_local_sales_tax -> Nullable<Numeric>,
        combo_sales_tax -> Nullable<Numeric>,
        state_use_tax -> Nullable<Numeric>,
        city_use_tax -> Nullable<Numeric>,
        city_local_use_tax -> Nullable<Numeric>,
        county_use_tax -> Nullable<Numeric>,
        county_local_use_tax -> Nullable<Numeric>,
        combo_use_tax -> Nullable<Numeric>,
        last_updated_stamp -> Nullable<Timestamptz>,
        last_updated_tx_stamp -> Nullable<Timestamptz>,
        created_stamp -> Nullable<Timestamptz>,
        created_tx_stamp -> Nullable<Timestamptz>,
    }
}

joinable!(accommodation_map -> accommodation_class (accommodation_class_id));
joinable!(accommodation_map -> accommodation_map_type (accommodation_map_type_id));
joinable!(accommodation_map -> fixed_asset (fixed_asset_id));
joinable!(accommodation_spot -> accommodation_class (accommodation_class_id));
joinable!(accommodation_spot -> fixed_asset (fixed_asset_id));
joinable!(acctg_trans -> acctg_trans_type (acctg_trans_type_id));
joinable!(acctg_trans -> fin_account_trans (fin_account_trans_id));
joinable!(acctg_trans -> fixed_asset (fixed_asset_id));
joinable!(acctg_trans -> gl_fiscal_type (gl_fiscal_type_id));
joinable!(acctg_trans -> gl_journal (gl_journal_id));
joinable!(acctg_trans -> inventory_item (inventory_item_id));
joinable!(acctg_trans -> invoice (invoice_id));
joinable!(acctg_trans -> party (party_id));
joinable!(acctg_trans -> payment (payment_id));
joinable!(acctg_trans -> physical_inventory (physical_inventory_id));
joinable!(acctg_trans -> role_type (role_type_id));
joinable!(acctg_trans -> shipment (shipment_id));
joinable!(acctg_trans -> shipment_receipt (receipt_id));
joinable!(acctg_trans -> status_item (group_status_id));
joinable!(acctg_trans -> work_effort (work_effort_id));
joinable!(acctg_trans_attribute -> acctg_trans (acctg_trans_id));
joinable!(acctg_trans_entry -> acctg_trans (acctg_trans_id));
joinable!(acctg_trans_entry -> acctg_trans_entry_type (acctg_trans_entry_type_id));
joinable!(acctg_trans_entry -> gl_account (gl_account_id));
joinable!(acctg_trans_entry -> gl_account_type (gl_account_type_id));
joinable!(acctg_trans_entry -> inventory_item (inventory_item_id));
joinable!(acctg_trans_entry -> party (party_id));
joinable!(acctg_trans_entry -> role_type (role_type_id));
joinable!(acctg_trans_entry -> settlement_term (settlement_term_id));
joinable!(acctg_trans_entry -> status_item (reconcile_status_id));
joinable!(acctg_trans_type_attr -> acctg_trans_type (acctg_trans_type_id));
joinable!(addendum -> agreement (agreement_id));
joinable!(affiliate -> party (party_id));
joinable!(affiliate -> party_group (party_id));
joinable!(agreement -> agreement_type (agreement_type_id));
joinable!(agreement -> product (product_id));
joinable!(agreement_attribute -> agreement (agreement_id));
joinable!(agreement_content -> agreement (agreement_id));
joinable!(agreement_content -> agreement_content_type (agreement_content_type_id));
joinable!(agreement_content -> content (content_id));
joinable!(agreement_facility_appl -> facility (facility_id));
joinable!(agreement_geographical_applic -> agreement (agreement_id));
joinable!(agreement_geographical_applic -> geo (geo_id));
joinable!(agreement_item -> agreement (agreement_id));
joinable!(agreement_item -> agreement_item_type (agreement_item_type_id));
joinable!(agreement_item_type_attr -> agreement_item_type (agreement_item_type_id));
joinable!(agreement_party_applic -> agreement (agreement_id));
joinable!(agreement_party_applic -> party (party_id));
joinable!(agreement_product_appl -> product (product_id));
joinable!(agreement_promo_appl -> product_promo (product_promo_id));
joinable!(agreement_role -> agreement (agreement_id));
joinable!(agreement_role -> party (party_id));
joinable!(agreement_term -> agreement (agreement_id));
joinable!(agreement_term -> invoice_item_type (invoice_item_type_id));
joinable!(agreement_term -> term_type (term_type_id));
joinable!(agreement_term_attribute -> agreement_term (agreement_term_id));
joinable!(agreement_type_attr -> agreement_type (agreement_type_id));
joinable!(agreement_work_effort_applic -> agreement (agreement_id));
joinable!(agreement_work_effort_applic -> work_effort (work_effort_id));
joinable!(allocation_plan_header -> allocation_plan_type (plan_type_id));
joinable!(allocation_plan_header -> status_item (status_id));
joinable!(allocation_plan_item -> enumeration (plan_method_enum_id));
joinable!(allocation_plan_item -> order_header (order_id));
joinable!(allocation_plan_item -> status_item (status_id));
joinable!(application_sandbox -> runtime_data (runtime_data_id));
joinable!(audio_data_resource -> data_resource (data_resource_id));
joinable!(billing_account -> contact_mech (contact_mech_id));
joinable!(billing_account -> postal_address (contact_mech_id));
joinable!(billing_account -> uom (account_currency_uom_id));
joinable!(billing_account_role -> billing_account (billing_account_id));
joinable!(billing_account_role -> party (party_id));
joinable!(billing_account_term -> billing_account (billing_account_id));
joinable!(billing_account_term -> term_type (term_type_id));
joinable!(billing_account_term -> uom (uom_id));
joinable!(billing_account_term_attr -> billing_account_term (billing_account_term_id));
joinable!(budget -> budget_type (budget_type_id));
joinable!(budget -> custom_time_period (custom_time_period_id));
joinable!(budget_attribute -> budget (budget_id));
joinable!(budget_item -> budget (budget_id));
joinable!(budget_item -> budget_item_type (budget_item_type_id));
joinable!(budget_item_type_attr -> budget_item_type (budget_item_type_id));
joinable!(budget_review -> budget (budget_id));
joinable!(budget_review -> budget_review_result_type (budget_review_result_type_id));
joinable!(budget_review -> party (party_id));
joinable!(budget_revision -> budget (budget_id));
joinable!(budget_revision_impact -> budget (budget_id));
joinable!(budget_role -> budget (budget_id));
joinable!(budget_role -> party (party_id));
joinable!(budget_scenario_application -> budget (budget_id));
joinable!(budget_scenario_application -> budget_scenario (budget_scenario_id));
joinable!(budget_scenario_rule -> budget_item_type (budget_item_type_id));
joinable!(budget_scenario_rule -> budget_scenario (budget_scenario_id));
joinable!(budget_status -> budget (budget_id));
joinable!(budget_status -> status_item (status_id));
joinable!(budget_status -> user_login (change_by_user_login_id));
joinable!(budget_type_attr -> budget_type (budget_type_id));
joinable!(carrier_shipment_box_type -> party (party_id));
joinable!(carrier_shipment_box_type -> shipment_box_type (shipment_box_type_id));
joinable!(carrier_shipment_method -> party (party_id));
joinable!(carrier_shipment_method -> shipment_method_type (shipment_method_type_id));
joinable!(cart_abandoned_line -> prod_catalog (prod_catalog_id));
joinable!(cart_abandoned_line -> product (product_id));
joinable!(check_account -> contact_mech (contact_mech_id));
joinable!(check_account -> payment_method (payment_method_id));
joinable!(check_account -> postal_address (contact_mech_id));
joinable!(comm_event_content_assoc -> comm_content_assoc_type (comm_content_assoc_type_id));
joinable!(comm_event_content_assoc -> communication_event (communication_event_id));
joinable!(comm_event_content_assoc -> content (content_id));
joinable!(communication_event -> communication_event_type (communication_event_type_id));
joinable!(communication_event -> contact_list (contact_list_id));
joinable!(communication_event -> contact_mech_type (contact_mech_type_id));
joinable!(communication_event -> enumeration (reason_enum_id));
joinable!(communication_event -> mime_type (content_mime_type_id));
joinable!(communication_event -> status_item (status_id));
joinable!(communication_event_order -> communication_event (communication_event_id));
joinable!(communication_event_order -> order_header (order_id));
joinable!(communication_event_product -> communication_event (communication_event_id));
joinable!(communication_event_product -> product (product_id));
joinable!(communication_event_purpose -> communication_event (communication_event_id));
joinable!(communication_event_purpose -> communication_event_prp_typ (communication_event_prp_typ_id));
joinable!(communication_event_return -> communication_event (communication_event_id));
joinable!(communication_event_return -> return_header (return_id));
joinable!(communication_event_role -> communication_event (communication_event_id));
joinable!(communication_event_role -> contact_mech (contact_mech_id));
joinable!(communication_event_role -> party (party_id));
joinable!(communication_event_role -> status_item (status_id));
joinable!(communication_event_type -> contact_mech_type (contact_mech_type_id));
joinable!(communication_event_work_eff -> communication_event (communication_event_id));
joinable!(communication_event_work_eff -> work_effort (work_effort_id));
joinable!(contact_list -> contact_list_type (contact_list_type_id));
joinable!(contact_list -> contact_mech_type (contact_mech_type_id));
joinable!(contact_list -> marketing_campaign (marketing_campaign_id));
joinable!(contact_list -> party (owner_party_id));
joinable!(contact_list_comm_status -> communication_event (communication_event_id));
joinable!(contact_list_comm_status -> contact_list (contact_list_id));
joinable!(contact_list_comm_status -> contact_mech (contact_mech_id));
joinable!(contact_list_comm_status -> party (party_id));
joinable!(contact_list_comm_status -> status_item (status_id));
joinable!(contact_list_comm_status -> user_login (change_by_user_login_id));
joinable!(contact_list_party -> contact_list (contact_list_id));
joinable!(contact_list_party -> contact_mech (preferred_contact_mech_id));
joinable!(contact_list_party -> party (party_id));
joinable!(contact_list_party -> status_item (status_id));
joinable!(contact_mech -> contact_mech_type (contact_mech_type_id));
joinable!(contact_mech_attribute -> contact_mech (contact_mech_id));
joinable!(contact_mech_type_attr -> contact_mech_type (contact_mech_type_id));
joinable!(contact_mech_type_purpose -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(contact_mech_type_purpose -> contact_mech_type (contact_mech_type_id));
joinable!(container -> container_type (container_type_id));
joinable!(container -> facility (facility_id));
joinable!(container_geo_point -> container (container_id));
joinable!(container_geo_point -> geo_point (geo_point_id));
joinable!(content -> character_set (character_set_id));
joinable!(content -> content_type (content_type_id));
joinable!(content -> custom_method (custom_method_id));
joinable!(content -> data_source (data_source_id));
joinable!(content -> enumeration (privilege_enum_id));
joinable!(content -> status_item (status_id));
joinable!(content_approval -> content (content_id));
joinable!(content_approval -> party (party_id));
joinable!(content_approval -> role_type (role_type_id));
joinable!(content_approval -> status_item (approval_status_id));
joinable!(content_assoc -> content_assoc_predicate (content_assoc_predicate_id));
joinable!(content_assoc -> content_assoc_type (content_assoc_type_id));
joinable!(content_assoc -> data_source (data_source_id));
joinable!(content_attribute -> content (content_id));
joinable!(content_keyword -> content (content_id));
joinable!(content_meta_data -> content (content_id));
joinable!(content_meta_data -> data_source (data_source_id));
joinable!(content_meta_data -> meta_data_predicate (meta_data_predicate_id));
joinable!(content_purpose -> content (content_id));
joinable!(content_purpose -> content_purpose_type (content_purpose_type_id));
joinable!(content_purpose_operation -> content_operation (content_operation_id));
joinable!(content_purpose_operation -> content_purpose_type (content_purpose_type_id));
joinable!(content_purpose_operation -> enumeration (privilege_enum_id));
joinable!(content_purpose_operation -> role_type (role_type_id));
joinable!(content_purpose_operation -> status_item (status_id));
joinable!(content_revision -> content (content_id));
joinable!(content_revision -> party (committed_by_party_id));
joinable!(content_role -> content (content_id));
joinable!(content_search_constraint -> content_search_result (content_search_result_id));
joinable!(content_type_attr -> content_type (content_type_id));
joinable!(cost_component -> cost_component_calc (cost_component_calc_id));
joinable!(cost_component -> cost_component_type (cost_component_type_id));
joinable!(cost_component -> fixed_asset (fixed_asset_id));
joinable!(cost_component -> geo (geo_id));
joinable!(cost_component -> party (party_id));
joinable!(cost_component -> product (product_id));
joinable!(cost_component -> product_feature (product_feature_id));
joinable!(cost_component -> uom (cost_uom_id));
joinable!(cost_component -> work_effort (work_effort_id));
joinable!(cost_component_attribute -> cost_component (cost_component_id));
joinable!(cost_component_calc -> custom_method (cost_custom_method_id));
joinable!(cost_component_calc -> uom (currency_uom_id));
joinable!(cost_component_type_attr -> cost_component_type (cost_component_type_id));
joinable!(country_address_format -> geo (geo_id));
joinable!(country_address_format -> geo_assoc_type (geo_assoc_type_id));
joinable!(country_capital -> country_code (country_code));
joinable!(country_tele_code -> country_code (country_code));
joinable!(credit_card -> contact_mech (contact_mech_id));
joinable!(credit_card -> payment_method (payment_method_id));
joinable!(credit_card -> postal_address (contact_mech_id));
joinable!(cust_request -> contact_mech (fulfill_contact_mech_id));
joinable!(cust_request -> cust_request_category (cust_request_category_id));
joinable!(cust_request -> cust_request_type (cust_request_type_id));
joinable!(cust_request -> enumeration (sales_channel_enum_id));
joinable!(cust_request -> party (from_party_id));
joinable!(cust_request -> product_store (product_store_id));
joinable!(cust_request -> status_item (status_id));
joinable!(cust_request_attribute -> cust_request (cust_request_id));
joinable!(cust_request_category -> cust_request_type (cust_request_type_id));
joinable!(cust_request_comm_event -> communication_event (communication_event_id));
joinable!(cust_request_comm_event -> cust_request (cust_request_id));
joinable!(cust_request_content -> content (content_id));
joinable!(cust_request_content -> cust_request (cust_request_id));
joinable!(cust_request_item -> cust_request (cust_request_id));
joinable!(cust_request_item -> cust_request_resolution (cust_request_resolution_id));
joinable!(cust_request_item -> product (product_id));
joinable!(cust_request_item -> status_item (status_id));
joinable!(cust_request_item_note -> note_data (note_id));
joinable!(cust_request_item_work_effort -> work_effort (work_effort_id));
joinable!(cust_request_note -> cust_request (cust_request_id));
joinable!(cust_request_note -> note_data (note_id));
joinable!(cust_request_party -> cust_request (cust_request_id));
joinable!(cust_request_party -> party (party_id));
joinable!(cust_request_resolution -> cust_request_type (cust_request_type_id));
joinable!(cust_request_status -> cust_request (cust_request_id));
joinable!(cust_request_status -> status_item (status_id));
joinable!(cust_request_status -> user_login (change_by_user_login_id));
joinable!(cust_request_type -> party (party_id));
joinable!(cust_request_type_attr -> cust_request_type (cust_request_type_id));
joinable!(cust_request_work_effort -> cust_request (cust_request_id));
joinable!(cust_request_work_effort -> work_effort (work_effort_id));
joinable!(custom_method -> custom_method_type (custom_method_type_id));
joinable!(custom_screen -> custom_screen_type (custom_screen_type_id));
joinable!(custom_time_period -> party (organization_party_id));
joinable!(custom_time_period -> period_type (period_type_id));
joinable!(data_resource -> character_set (character_set_id));
joinable!(data_resource -> data_category (data_category_id));
joinable!(data_resource -> data_resource_type (data_resource_type_id));
joinable!(data_resource -> data_source (data_source_id));
joinable!(data_resource -> data_template_type (data_template_type_id));
joinable!(data_resource -> status_item (status_id));
joinable!(data_resource -> survey (survey_id));
joinable!(data_resource -> survey_response (survey_response_id));
joinable!(data_resource_attribute -> data_resource (data_resource_id));
joinable!(data_resource_meta_data -> data_resource (data_resource_id));
joinable!(data_resource_meta_data -> data_source (data_source_id));
joinable!(data_resource_meta_data -> meta_data_predicate (meta_data_predicate_id));
joinable!(data_resource_purpose -> content_purpose_type (content_purpose_type_id));
joinable!(data_resource_purpose -> data_resource (data_resource_id));
joinable!(data_resource_role -> data_resource (data_resource_id));
joinable!(data_resource_type_attr -> data_resource_type (data_resource_type_id));
joinable!(data_source -> data_source_type (data_source_type_id));
joinable!(deduction -> deduction_type (deduction_type_id));
joinable!(deduction -> payment (payment_id));
joinable!(deliverable -> deliverable_type (deliverable_type_id));
joinable!(delivery -> fixed_asset (fixed_asset_id));
joinable!(desired_feature -> product_feature (product_feature_id));
joinable!(desired_feature -> requirement (requirement_id));
joinable!(document -> document_type (document_type_id));
joinable!(document_attribute -> document (document_id));
joinable!(document_type_attr -> document_type (document_type_id));
joinable!(ebay_config -> product_store (product_store_id));
joinable!(ebay_config -> web_site (web_site_id));
joinable!(ebay_shipping_method -> enumeration (method_type_enum_id));
joinable!(ebay_shipping_method -> party (carrier_party_id));
joinable!(ebay_shipping_method -> product_store (product_store_id));
joinable!(ebay_shipping_method -> shipment_method_type (shipment_method_type_id));
joinable!(eft_account -> contact_mech (contact_mech_id));
joinable!(eft_account -> payment_method (payment_method_id));
joinable!(eft_account -> postal_address (contact_mech_id));
joinable!(electronic_text -> data_resource (data_resource_id));
joinable!(email_template_setting -> enumeration (email_type));
joinable!(empl_leave -> empl_leave_reason_type (empl_leave_reason_type_id));
joinable!(empl_leave -> empl_leave_type (leave_type_id));
joinable!(empl_leave -> status_item (leave_status));
joinable!(empl_position -> party (party_id));
joinable!(empl_position -> status_item (status_id));
joinable!(empl_position_fulfillment -> empl_position (empl_position_id));
joinable!(empl_position_fulfillment -> party (party_id));
joinable!(empl_position_responsibility -> empl_position (empl_position_id));
joinable!(empl_position_responsibility -> responsibility_type (responsibility_type_id));
joinable!(empl_position_type_class -> empl_position_class_type (empl_position_class_type_id));
joinable!(empl_position_type_class -> empl_position_type (empl_position_type_id));
joinable!(empl_position_type_rate_new -> empl_position_type (empl_position_type_id));
joinable!(employment_app -> job_requisition (job_requisition_id));
joinable!(employment_app -> party (approver_party_id));
joinable!(entity_group_entry -> entity_group (entity_group_id));
joinable!(entity_sync_history -> entity_sync (entity_sync_id));
joinable!(entity_sync_include -> entity_sync (entity_sync_id));
joinable!(entity_sync_include_group -> entity_group (entity_group_id));
joinable!(entity_sync_include_group -> entity_sync (entity_sync_id));
joinable!(enumeration -> enumeration_type (enum_type_id));
joinable!(example -> example_type (example_type_id));
joinable!(example -> status_item (status_id));
joinable!(example_feature -> enumeration (feature_source_enum_id));
joinable!(example_feature_appl -> example (example_id));
joinable!(example_feature_appl -> example_feature (example_feature_id));
joinable!(example_feature_appl -> example_feature_appl_type (example_feature_appl_type_id));
joinable!(example_item -> example (example_id));
joinable!(example_item -> uom (amount_uom_id));
joinable!(example_status -> example (example_id));
joinable!(example_status -> status_item (status_id));
joinable!(example_status -> user_login (change_by_user_login_id));
joinable!(excel_import_history -> user_login (user_login_id));
joinable!(facility -> facility_group (primary_facility_group_id));
joinable!(facility -> facility_type (facility_type_id));
joinable!(facility -> geo_point (geo_point_id));
joinable!(facility -> inventory_item_type (default_inventory_item_type_id));
joinable!(facility -> party (owner_party_id));
joinable!(facility_attribute -> facility (facility_id));
joinable!(facility_calendar -> facility (facility_id));
joinable!(facility_calendar -> facility_calendar_type (facility_calendar_type_id));
joinable!(facility_carrier_shipment -> facility (facility_id));
joinable!(facility_carrier_shipment -> party (party_id));
joinable!(facility_carrier_shipment -> shipment_method_type (shipment_method_type_id));
joinable!(facility_contact_mech -> contact_mech (contact_mech_id));
joinable!(facility_contact_mech -> facility (facility_id));
joinable!(facility_contact_mech_purpose -> contact_mech (contact_mech_id));
joinable!(facility_contact_mech_purpose -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(facility_contact_mech_purpose -> facility (facility_id));
joinable!(facility_content -> content (content_id));
joinable!(facility_content -> facility (facility_id));
joinable!(facility_group -> facility_group_type (facility_group_type_id));
joinable!(facility_group_member -> facility (facility_id));
joinable!(facility_group_member -> facility_group (facility_group_id));
joinable!(facility_group_role -> facility_group (facility_group_id));
joinable!(facility_location -> enumeration (location_type_enum_id));
joinable!(facility_location -> facility (facility_id));
joinable!(facility_location -> geo_point (geo_point_id));
joinable!(facility_location_geo_point -> geo_point (geo_point_id));
joinable!(facility_party -> facility (facility_id));
joinable!(facility_party -> party (party_id));
joinable!(facility_party -> role_type (role_type_id));
joinable!(facility_type_attr -> facility_type (facility_type_id));
joinable!(file_extension -> mime_type (mime_type_id));
joinable!(fin_account -> fin_account_type (fin_account_type_id));
joinable!(fin_account -> gl_account (post_to_gl_account_id));
joinable!(fin_account -> uom (currency_uom_id));
joinable!(fin_account_attribute -> fin_account (fin_account_id));
joinable!(fin_account_auth -> fin_account (fin_account_id));
joinable!(fin_account_role -> fin_account (fin_account_id));
joinable!(fin_account_status -> fin_account (fin_account_id));
joinable!(fin_account_status -> status_item (status_id));
joinable!(fin_account_status -> user_login (change_by_user_login_id));
joinable!(fin_account_trans -> enumeration (reason_enum_id));
joinable!(fin_account_trans -> fin_account (fin_account_id));
joinable!(fin_account_trans -> fin_account_trans_type (fin_account_trans_type_id));
joinable!(fin_account_trans -> gl_reconciliation (gl_reconciliation_id));
joinable!(fin_account_trans -> status_item (status_id));
joinable!(fin_account_trans_attribute -> fin_account_trans (fin_account_trans_id));
joinable!(fin_account_trans_type_attr -> fin_account_trans_type (fin_account_trans_type_id));
joinable!(fin_account_type -> enumeration (replenish_enum_id));
joinable!(fin_account_type_attr -> fin_account_type (fin_account_type_id));
joinable!(fin_account_type_gl_account -> fin_account_type (fin_account_type_id));
joinable!(fin_account_type_gl_account -> gl_account (gl_account_id));
joinable!(fin_account_type_gl_account -> party (organization_party_id));
joinable!(fixed_asset -> enumeration (class_enum_id));
joinable!(fixed_asset -> facility (located_at_facility_id));
joinable!(fixed_asset -> fixed_asset_type (fixed_asset_type_id));
joinable!(fixed_asset -> order_header (acquire_order_id));
joinable!(fixed_asset -> party (party_id));
joinable!(fixed_asset -> product (instance_of_product_id));
joinable!(fixed_asset -> role_type (role_type_id));
joinable!(fixed_asset -> tech_data_calendar (calendar_id));
joinable!(fixed_asset -> uom (uom_id));
joinable!(fixed_asset_attribute -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_dep_method -> custom_method (depreciation_custom_method_id));
joinable!(fixed_asset_dep_method -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_geo_point -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_geo_point -> geo_point (geo_point_id));
joinable!(fixed_asset_ident -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_ident -> fixed_asset_ident_type (fixed_asset_ident_type_id));
joinable!(fixed_asset_maint -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_maint -> order_header (purchase_order_id));
joinable!(fixed_asset_maint -> product_maint_type (product_maint_type_id));
joinable!(fixed_asset_maint -> product_meter_type (interval_meter_type_id));
joinable!(fixed_asset_maint -> status_item (status_id));
joinable!(fixed_asset_maint -> uom (interval_uom_id));
joinable!(fixed_asset_maint -> work_effort (schedule_work_effort_id));
joinable!(fixed_asset_maint_order -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_maint_order -> order_header (order_id));
joinable!(fixed_asset_meter -> product_meter_type (product_meter_type_id));
joinable!(fixed_asset_product -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_product -> fixed_asset_product_type (fixed_asset_product_type_id));
joinable!(fixed_asset_product -> product (product_id));
joinable!(fixed_asset_product -> uom (quantity_uom_id));
joinable!(fixed_asset_registration -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_registration -> party (gov_agency_party_id));
joinable!(fixed_asset_std_cost -> fixed_asset (fixed_asset_id));
joinable!(fixed_asset_std_cost -> fixed_asset_std_cost_type (fixed_asset_std_cost_type_id));
joinable!(fixed_asset_std_cost -> uom (amount_uom_id));
joinable!(fixed_asset_type_attr -> fixed_asset_type (fixed_asset_type_id));
joinable!(fixed_asset_type_gl_account -> party (organization_party_id));
joinable!(ftp_address -> contact_mech (contact_mech_id));
joinable!(geo -> geo_type (geo_type_id));
joinable!(geo_assoc -> geo_assoc_type (geo_assoc_type_id));
joinable!(geo_point -> data_source (data_source_id));
joinable!(geo_point -> enumeration (geo_point_type_enum_id));
joinable!(geo_point -> uom (elevation_uom_id));
joinable!(gift_card -> contact_mech (contact_mech_id));
joinable!(gift_card -> payment_method (payment_method_id));
joinable!(gift_card -> postal_address (contact_mech_id));
joinable!(gift_card_fulfillment -> enumeration (type_enum_id));
joinable!(gift_card_fulfillment -> order_header (order_id));
joinable!(gift_card_fulfillment -> party (party_id));
joinable!(gift_card_fulfillment -> survey_response (survey_response_id));
joinable!(git_hub_user -> product_store (product_store_id));
joinable!(gl_account -> gl_account_class (gl_account_class_id));
joinable!(gl_account -> gl_account_type (gl_account_type_id));
joinable!(gl_account -> gl_resource_type (gl_resource_type_id));
joinable!(gl_account -> gl_xbrl_class (gl_xbrl_class_id));
joinable!(gl_account_category -> gl_account_category_type (gl_account_category_type_id));
joinable!(gl_account_category_member -> gl_account (gl_account_id));
joinable!(gl_account_category_member -> gl_account_category (gl_account_category_id));
joinable!(gl_account_group -> gl_account_group_type (gl_account_group_type_id));
joinable!(gl_account_group_member -> gl_account (gl_account_id));
joinable!(gl_account_group_member -> gl_account_group (gl_account_group_id));
joinable!(gl_account_group_member -> gl_account_group_type (gl_account_group_type_id));
joinable!(gl_account_history -> custom_time_period (custom_time_period_id));
joinable!(gl_account_history -> gl_account (gl_account_id));
joinable!(gl_account_history -> party (organization_party_id));
joinable!(gl_account_organization -> gl_account (gl_account_id));
joinable!(gl_account_organization -> party (organization_party_id));
joinable!(gl_account_role -> gl_account (gl_account_id));
joinable!(gl_account_type_default -> gl_account (gl_account_id));
joinable!(gl_account_type_default -> gl_account_type (gl_account_type_id));
joinable!(gl_account_type_default -> party (organization_party_id));
joinable!(gl_budget_xref -> budget_item_type (budget_item_type_id));
joinable!(gl_budget_xref -> gl_account (gl_account_id));
joinable!(gl_journal -> party (organization_party_id));
joinable!(gl_reconciliation -> gl_account (gl_account_id));
joinable!(gl_reconciliation -> party (organization_party_id));
joinable!(gl_reconciliation -> status_item (status_id));
joinable!(gl_reconciliation_entry -> gl_reconciliation (gl_reconciliation_id));
joinable!(good_identification -> good_identification_type (good_identification_type_id));
joinable!(good_identification -> product (product_id));
joinable!(image_data_resource -> data_resource (data_resource_id));
joinable!(inventory_item -> container (container_id));
joinable!(inventory_item -> facility (facility_id));
joinable!(inventory_item -> fixed_asset (fixed_asset_id));
joinable!(inventory_item -> inventory_item_type (inventory_item_type_id));
joinable!(inventory_item -> lot (lot_id));
joinable!(inventory_item -> product (product_id));
joinable!(inventory_item -> status_item (status_id));
joinable!(inventory_item_attribute -> inventory_item (inventory_item_id));
joinable!(inventory_item_detail -> enumeration (reason_enum_id));
joinable!(inventory_item_detail -> inventory_item (inventory_item_id));
joinable!(inventory_item_detail -> item_issuance (item_issuance_id));
joinable!(inventory_item_detail -> physical_inventory (physical_inventory_id));
joinable!(inventory_item_detail -> shipment_receipt (receipt_id));
joinable!(inventory_item_detail -> work_effort (work_effort_id));
joinable!(inventory_item_label -> inventory_item_label_type (inventory_item_label_type_id));
joinable!(inventory_item_label_appl -> inventory_item (inventory_item_id));
joinable!(inventory_item_label_appl -> inventory_item_label (inventory_item_label_id));
joinable!(inventory_item_label_appl -> inventory_item_label_type (inventory_item_label_type_id));
joinable!(inventory_item_status -> inventory_item (inventory_item_id));
joinable!(inventory_item_status -> status_item (status_id));
joinable!(inventory_item_status -> user_login (change_by_user_login_id));
joinable!(inventory_item_temp_res -> product (product_id));
joinable!(inventory_item_temp_res -> product_store (product_store_id));
joinable!(inventory_item_type_attr -> inventory_item_type (inventory_item_type_id));
joinable!(inventory_item_variance -> inventory_item (inventory_item_id));
joinable!(inventory_item_variance -> physical_inventory (physical_inventory_id));
joinable!(inventory_item_variance -> variance_reason (variance_reason_id));
joinable!(inventory_transfer -> inventory_item (inventory_item_id));
joinable!(inventory_transfer -> item_issuance (item_issuance_id));
joinable!(inventory_transfer -> status_item (status_id));
joinable!(invoice -> billing_account (billing_account_id));
joinable!(invoice -> contact_mech (contact_mech_id));
joinable!(invoice -> invoice_type (invoice_type_id));
joinable!(invoice -> recurrence_info (recurrence_info_id));
joinable!(invoice -> role_type (role_type_id));
joinable!(invoice -> status_item (status_id));
joinable!(invoice -> uom (currency_uom_id));
joinable!(invoice_attribute -> invoice (invoice_id));
joinable!(invoice_contact_mech -> contact_mech (contact_mech_id));
joinable!(invoice_contact_mech -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(invoice_contact_mech -> invoice (invoice_id));
joinable!(invoice_content -> content (content_id));
joinable!(invoice_content -> invoice (invoice_id));
joinable!(invoice_content -> invoice_content_type (invoice_content_type_id));
joinable!(invoice_item -> geo (tax_auth_geo_id));
joinable!(invoice_item -> gl_account (override_gl_account_id));
joinable!(invoice_item -> inventory_item (inventory_item_id));
joinable!(invoice_item -> invoice (invoice_id));
joinable!(invoice_item -> invoice_item_type (invoice_item_type_id));
joinable!(invoice_item -> product (product_id));
joinable!(invoice_item -> product_feature (product_feature_id));
joinable!(invoice_item -> sales_opportunity (sales_opportunity_id));
joinable!(invoice_item -> tax_authority_rate_product (tax_authority_rate_seq_id));
joinable!(invoice_item -> uom (uom_id));
joinable!(invoice_item_assoc -> invoice_item_assoc_type (invoice_item_assoc_type_id));
joinable!(invoice_item_type -> gl_account (default_gl_account_id));
joinable!(invoice_item_type_attr -> invoice_item_type (invoice_item_type_id));
joinable!(invoice_item_type_gl_account -> gl_account (gl_account_id));
joinable!(invoice_item_type_gl_account -> invoice_item_type (invoice_item_type_id));
joinable!(invoice_item_type_gl_account -> party (organization_party_id));
joinable!(invoice_item_type_map -> invoice_item_type (invoice_item_type_id));
joinable!(invoice_item_type_map -> invoice_type (invoice_type_id));
joinable!(invoice_note -> invoice (invoice_id));
joinable!(invoice_note -> note_data (note_id));
joinable!(invoice_role -> invoice (invoice_id));
joinable!(invoice_role -> party (party_id));
joinable!(invoice_status -> invoice (invoice_id));
joinable!(invoice_status -> status_item (status_id));
joinable!(invoice_status -> user_login (change_by_user_login_id));
joinable!(invoice_term -> invoice (invoice_id));
joinable!(invoice_term -> term_type (term_type_id));
joinable!(invoice_term_attribute -> invoice_term (invoice_term_id));
joinable!(invoice_type_attr -> invoice_type (invoice_type_id));
joinable!(item_issuance -> inventory_item (inventory_item_id));
joinable!(item_issuance -> user_login (issued_by_user_login_id));
joinable!(item_issuance_role -> item_issuance (item_issuance_id));
joinable!(item_issuance_role -> party (party_id));
joinable!(job_interview -> enumeration (grade_secured_enum_id));
joinable!(job_interview -> job_interview_type (job_interview_type_id));
joinable!(job_interview -> job_requisition (job_requisition_id));
joinable!(job_manager_lock -> enumeration (reason_enum_id));
joinable!(job_requisition -> skill_type (skill_type_id));
joinable!(job_sandbox -> recurrence_info (recurrence_info_id));
joinable!(job_sandbox -> runtime_data (runtime_data_id));
joinable!(job_sandbox -> status_item (status_id));
joinable!(job_sandbox -> temporal_expression (temp_expr_id));
joinable!(keyword_thesaurus -> enumeration (relationship_enum_id));
joinable!(linked_in_user -> product_store (product_store_id));
joinable!(market_interest -> party_classification_group (party_classification_group_id));
joinable!(market_interest -> product_category (product_category_id));
joinable!(marketing_campaign -> status_item (status_id));
joinable!(marketing_campaign -> uom (currency_uom_id));
joinable!(marketing_campaign_note -> marketing_campaign (marketing_campaign_id));
joinable!(marketing_campaign_note -> note_data (note_id));
joinable!(marketing_campaign_price -> marketing_campaign (marketing_campaign_id));
joinable!(marketing_campaign_price -> product_price_rule (product_price_rule_id));
joinable!(marketing_campaign_promo -> marketing_campaign (marketing_campaign_id));
joinable!(marketing_campaign_promo -> product_promo (product_promo_id));
joinable!(marketing_campaign_role -> marketing_campaign (marketing_campaign_id));
joinable!(mime_type_html_template -> mime_type (mime_type_id));
joinable!(mrp_event -> facility (facility_id));
joinable!(mrp_event -> mrp_event_type (mrp_event_type_id));
joinable!(mrp_event -> product (product_id));
joinable!(msg91_gateway_config -> telecom_gateway_config (telecom_gateway_config_id));
joinable!(note_data -> party (note_party));
joinable!(o_auth2_git_hub -> product_store (product_store_id));
joinable!(o_auth2_linked_in -> product_store (product_store_id));
joinable!(order_adjustment -> gl_account (override_gl_account_id));
joinable!(order_adjustment -> order_adjustment_type (order_adjustment_type_id));
joinable!(order_adjustment -> order_header (order_id));
joinable!(order_adjustment -> product_promo (product_promo_id));
joinable!(order_adjustment -> tax_authority_rate_product (tax_authority_rate_seq_id));
joinable!(order_adjustment -> user_login (created_by_user_login));
joinable!(order_adjustment_attribute -> order_adjustment (order_adjustment_id));
joinable!(order_adjustment_billing -> order_adjustment (order_adjustment_id));
joinable!(order_adjustment_type_attr -> order_adjustment_type (order_adjustment_type_id));
joinable!(order_attribute -> order_header (order_id));
joinable!(order_blacklist -> order_blacklist_type (order_blacklist_type_id));
joinable!(order_contact_mech -> contact_mech (contact_mech_id));
joinable!(order_contact_mech -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(order_contact_mech -> order_header (order_id));
joinable!(order_content -> content (content_id));
joinable!(order_content -> order_content_type (order_content_type_id));
joinable!(order_content -> order_header (order_id));
joinable!(order_delivery_schedule -> order_header (order_id));
joinable!(order_delivery_schedule -> status_item (status_id));
joinable!(order_denylist -> order_denylist_type (order_denylist_type_id));
joinable!(order_header -> billing_account (billing_account_id));
joinable!(order_header -> enumeration (sales_channel_enum_id));
joinable!(order_header -> facility (origin_facility_id));
joinable!(order_header -> order_type (order_type_id));
joinable!(order_header -> product_store (product_store_id));
joinable!(order_header -> shopping_list (auto_order_shopping_list_id));
joinable!(order_header -> uom (currency_uom));
joinable!(order_header -> user_login (created_by));
joinable!(order_header -> web_site (web_site_id));
joinable!(order_header_note -> note_data (note_id));
joinable!(order_header_note -> order_header (order_id));
joinable!(order_header_work_effort -> order_header (order_id));
joinable!(order_header_work_effort -> work_effort (work_effort_id));
joinable!(order_item -> gl_account (override_gl_account_id));
joinable!(order_item -> inventory_item (from_inventory_item_id));
joinable!(order_item -> order_header (order_id));
joinable!(order_item -> order_item_type (order_item_type_id));
joinable!(order_item -> product (product_id));
joinable!(order_item -> sales_opportunity (sales_opportunity_id));
joinable!(order_item -> uom (recurring_freq_uom_id));
joinable!(order_item_assoc -> order_item_assoc_type (order_item_assoc_type_id));
joinable!(order_item_billing -> item_issuance (item_issuance_id));
joinable!(order_item_billing -> order_header (order_id));
joinable!(order_item_billing -> shipment_receipt (shipment_receipt_id));
joinable!(order_item_change -> user_login (change_user_login));
joinable!(order_item_contact_mech -> contact_mech (contact_mech_id));
joinable!(order_item_contact_mech -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(order_item_group -> order_header (order_id));
joinable!(order_item_group_order -> product_group_order (group_order_id));
joinable!(order_item_role -> order_header (order_id));
joinable!(order_item_role -> party (party_id));
joinable!(order_item_ship_group -> agreement (supplier_agreement_id));
joinable!(order_item_ship_group -> facility (facility_id));
joinable!(order_item_ship_group -> order_header (order_id));
joinable!(order_item_ship_group -> postal_address (contact_mech_id));
joinable!(order_item_ship_group -> shipment_method_type (shipment_method_type_id));
joinable!(order_item_ship_group -> telecom_number (telecom_contact_mech_id));
joinable!(order_item_ship_group_assoc -> order_header (order_id));
joinable!(order_item_ship_grp_inv_res -> inventory_item (inventory_item_id));
joinable!(order_item_type_attr -> order_item_type (order_item_type_id));
joinable!(order_notification -> enumeration (email_type));
joinable!(order_notification -> order_header (order_id));
joinable!(order_payment_preference -> fin_account (fin_account_id));
joinable!(order_payment_preference -> order_header (order_id));
joinable!(order_payment_preference -> payment_method (payment_method_id));
joinable!(order_payment_preference -> payment_method_type (payment_method_type_id));
joinable!(order_payment_preference -> product_price_purpose (product_price_purpose_id));
joinable!(order_payment_preference -> status_item (status_id));
joinable!(order_payment_preference -> user_login (created_by_user_login));
joinable!(order_product_promo_code -> order_header (order_id));
joinable!(order_product_promo_code -> product_promo_code (product_promo_code_id));
joinable!(order_requirement_commitment -> order_header (order_id));
joinable!(order_requirement_commitment -> requirement (requirement_id));
joinable!(order_role -> order_header (order_id));
joinable!(order_role -> party (party_id));
joinable!(order_shipment -> order_header (order_id));
joinable!(order_shipment -> shipment (shipment_id));
joinable!(order_status -> order_header (order_id));
joinable!(order_status -> status_item (status_id));
joinable!(order_status -> user_login (status_user_login));
joinable!(order_summary_entry -> facility (facility_id));
joinable!(order_summary_entry -> product (product_id));
joinable!(order_term -> order_header (order_id));
joinable!(order_term -> term_type (term_type_id));
joinable!(order_term -> uom (uom_id));
joinable!(order_type_attr -> order_type (order_type_id));
joinable!(other_data_resource -> data_resource (data_resource_id));
joinable!(party -> data_source (data_source_id));
joinable!(party -> party_type (party_type_id));
joinable!(party -> status_item (status_id));
joinable!(party -> uom (preferred_currency_uom_id));
joinable!(party_acctg_preference -> gl_journal (error_gl_journal_id));
joinable!(party_acctg_preference -> party (party_id));
joinable!(party_acctg_preference -> payment_method (refund_payment_method_id));
joinable!(party_acctg_preference -> uom (base_currency_uom_id));
joinable!(party_attribute -> party (party_id));
joinable!(party_benefit -> benefit_type (benefit_type_id));
joinable!(party_classification -> party (party_id));
joinable!(party_classification -> party_classification_group (party_classification_group_id));
joinable!(party_classification_group -> party_classification_type (party_classification_type_id));
joinable!(party_contact_mech -> contact_mech (contact_mech_id));
joinable!(party_contact_mech -> party (party_id));
joinable!(party_contact_mech -> role_type (role_type_id));
joinable!(party_contact_mech_purpose -> contact_mech (contact_mech_id));
joinable!(party_contact_mech_purpose -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(party_contact_mech_purpose -> party (party_id));
joinable!(party_content -> content (content_id));
joinable!(party_content -> party (party_id));
joinable!(party_content -> party_content_type (party_content_type_id));
joinable!(party_data_source -> data_source (data_source_id));
joinable!(party_data_source -> party (party_id));
joinable!(party_fixed_asset_assignment -> fixed_asset (fixed_asset_id));
joinable!(party_fixed_asset_assignment -> status_item (status_id));
joinable!(party_geo_point -> geo_point (geo_point_id));
joinable!(party_geo_point -> party (party_id));
joinable!(party_gl_account -> gl_account (gl_account_id));
joinable!(party_gl_account -> gl_account_type (gl_account_type_id));
joinable!(party_group -> party (party_id));
joinable!(party_ics_avs_override -> party (party_id));
joinable!(party_identification -> party (party_id));
joinable!(party_identification -> party_identification_type (party_identification_type_id));
joinable!(party_invitation -> party (party_id_from));
joinable!(party_invitation -> status_item (status_id));
joinable!(party_invitation_group_assoc -> party (party_id_to));
joinable!(party_invitation_group_assoc -> party_group (party_id_to));
joinable!(party_invitation_group_assoc -> party_invitation (party_invitation_id));
joinable!(party_invitation_role_assoc -> party_invitation (party_invitation_id));
joinable!(party_invitation_role_assoc -> role_type (role_type_id));
joinable!(party_name_history -> party (party_id));
joinable!(party_need -> communication_event (communication_event_id));
joinable!(party_need -> need_type (need_type_id));
joinable!(party_need -> party (party_id));
joinable!(party_need -> party_type (party_type_id));
joinable!(party_need -> product (product_id));
joinable!(party_need -> product_category (product_category_id));
joinable!(party_need -> role_type (role_type_id));
joinable!(party_note -> note_data (note_id));
joinable!(party_note -> party (party_id));
joinable!(party_pref_doc_type_tpl -> invoice_type (invoice_type_id));
joinable!(party_pref_doc_type_tpl -> order_type (order_type_id));
joinable!(party_pref_doc_type_tpl -> party (party_id));
joinable!(party_pref_doc_type_tpl -> party_acctg_preference (party_id));
joinable!(party_pref_doc_type_tpl -> quote_type (quote_type_id));
joinable!(party_profile_default -> party (party_id));
joinable!(party_profile_default -> product_store (product_store_id));
joinable!(party_qual -> party (party_id));
joinable!(party_qual -> party_qual_type (party_qual_type_id));
joinable!(party_rate_new -> party (party_id));
joinable!(party_rate_new -> rate_type (rate_type_id));
joinable!(party_relationship -> party_relationship_type (party_relationship_type_id));
joinable!(party_relationship -> priority_type (priority_type_id));
joinable!(party_relationship -> security_group (security_group_id));
joinable!(party_relationship -> status_item (status_id));
joinable!(party_resume -> party (party_id));
joinable!(party_role -> party (party_id));
joinable!(party_role -> role_type (role_type_id));
joinable!(party_skill -> party (party_id));
joinable!(party_skill -> skill_type (skill_type_id));
joinable!(party_status -> party (party_id));
joinable!(party_status -> status_item (status_id));
joinable!(party_status -> user_login (change_by_user_login_id));
joinable!(party_tax_auth_info -> party (party_id));
joinable!(party_type_attr -> party_type (party_type_id));
joinable!(pay_history -> pay_grade (pay_grade_id));
joinable!(pay_history -> period_type (period_type_id));
joinable!(pay_pal_payment_method -> contact_mech (contact_mech_id));
joinable!(pay_pal_payment_method -> payment_method (payment_method_id));
joinable!(pay_pal_payment_method -> postal_address (contact_mech_id));
joinable!(payment -> gl_account (override_gl_account_id));
joinable!(payment -> order_payment_preference (payment_preference_id));
joinable!(payment -> payment_gateway_response (payment_gateway_response_id));
joinable!(payment -> payment_method (payment_method_id));
joinable!(payment -> payment_method_type (payment_method_type_id));
joinable!(payment -> payment_type (payment_type_id));
joinable!(payment -> role_type (role_type_id_to));
joinable!(payment -> status_item (status_id));
joinable!(payment_application -> billing_account (billing_account_id));
joinable!(payment_application -> geo (tax_auth_geo_id));
joinable!(payment_application -> gl_account (override_gl_account_id));
joinable!(payment_application -> invoice (invoice_id));
joinable!(payment_attribute -> payment (payment_id));
joinable!(payment_budget_allocation -> budget (budget_id));
joinable!(payment_budget_allocation -> payment (payment_id));
joinable!(payment_content -> content (content_id));
joinable!(payment_content -> payment (payment_id));
joinable!(payment_content -> payment_content_type (payment_content_type_id));
joinable!(payment_gateway_authorize_net -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_clear_commerce -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_config -> payment_gateway_config_type (payment_gateway_config_type_id));
joinable!(payment_gateway_cyber_source -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_eway -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_first_data -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_orbital -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_pay_pal -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_payflow_pro -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_resp_msg -> payment_gateway_response (payment_gateway_response_id));
joinable!(payment_gateway_response -> order_payment_preference (order_payment_preference_id));
joinable!(payment_gateway_response -> payment_method (payment_method_id));
joinable!(payment_gateway_response -> payment_method_type (payment_method_type_id));
joinable!(payment_gateway_response -> uom (currency_uom_id));
joinable!(payment_gateway_sage_pay -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_secure_pay -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gateway_world_pay -> payment_gateway_config (payment_gateway_config_id));
joinable!(payment_gl_account_type_map -> gl_account_type (gl_account_type_id));
joinable!(payment_gl_account_type_map -> party (organization_party_id));
joinable!(payment_gl_account_type_map -> payment_type (payment_type_id));
joinable!(payment_group -> payment_group_type (payment_group_type_id));
joinable!(payment_group_member -> payment (payment_id));
joinable!(payment_group_member -> payment_group (payment_group_id));
joinable!(payment_method -> gl_account (gl_account_id));
joinable!(payment_method -> party (party_id));
joinable!(payment_method -> payment_method_type (payment_method_type_id));
joinable!(payment_method_type -> gl_account (default_gl_account_id));
joinable!(payment_method_type_gl_account -> gl_account (gl_account_id));
joinable!(payment_method_type_gl_account -> party (organization_party_id));
joinable!(payment_method_type_gl_account -> payment_method_type (payment_method_type_id));
joinable!(payment_type_attr -> payment_type (payment_type_id));
joinable!(payroll_preference -> party (party_id));
joinable!(perf_review -> payment (payment_id));
joinable!(perf_review_item -> party (employee_party_id));
joinable!(performance_note -> party (party_id));
joinable!(period_type -> uom (uom_id));
joinable!(person -> party (party_id));
joinable!(person_training -> party (party_id));
joinable!(person_training -> person (approver_id));
joinable!(person_training -> training_class_type (training_class_type_id));
joinable!(person_training -> training_request (training_request_id));
joinable!(person_training -> work_effort (work_effort_id));
joinable!(picklist -> facility (facility_id));
joinable!(picklist -> shipment_method_type (shipment_method_type_id));
joinable!(picklist -> status_item (status_id));
joinable!(picklist_bin -> picklist (picklist_id));
joinable!(picklist_item -> inventory_item (inventory_item_id));
joinable!(picklist_item -> picklist_bin (picklist_bin_id));
joinable!(picklist_item -> status_item (item_status_id));
joinable!(picklist_role -> picklist (picklist_id));
joinable!(picklist_status -> picklist (picklist_id));
joinable!(picklist_status -> user_login (change_by_user_login_id));
joinable!(picklist_status_history -> picklist (picklist_id));
joinable!(picklist_status_history -> user_login (change_user_login_id));
joinable!(portal_page -> content (help_content_id));
joinable!(portal_page -> security_group (security_group_id));
joinable!(portal_page_column -> portal_page (portal_page_id));
joinable!(portal_page_portlet -> portal_page (portal_page_id));
joinable!(portal_page_portlet -> portal_portlet (portal_portlet_id));
joinable!(portlet_attribute -> portal_portlet (portal_portlet_id));
joinable!(portlet_portlet_category -> portal_portlet (portal_portlet_id));
joinable!(portlet_portlet_category -> portlet_category (portlet_category_id));
joinable!(pos_terminal_intern_tx -> enumeration (reason_enum_id));
joinable!(pos_terminal_intern_tx -> pos_terminal_log (pos_terminal_log_id));
joinable!(pos_terminal_log -> order_header (order_id));
joinable!(pos_terminal_log -> pos_terminal (pos_terminal_id));
joinable!(pos_terminal_log -> return_header (return_id));
joinable!(pos_terminal_log -> status_item (status_id));
joinable!(pos_terminal_state -> pos_terminal (pos_terminal_id));
joinable!(postal_address -> contact_mech (contact_mech_id));
joinable!(postal_address -> geo_point (geo_point_id));
joinable!(postal_address_boundary -> geo (geo_id));
joinable!(postal_address_boundary -> postal_address (contact_mech_id));
joinable!(prod_catalog_category -> prod_catalog (prod_catalog_id));
joinable!(prod_catalog_category -> prod_catalog_category_type (prod_catalog_category_type_id));
joinable!(prod_catalog_category -> product_category (product_category_id));
joinable!(prod_catalog_inv_facility -> facility (facility_id));
joinable!(prod_catalog_inv_facility -> prod_catalog (prod_catalog_id));
joinable!(prod_catalog_role -> prod_catalog (prod_catalog_id));
joinable!(prod_conf_item_content -> content (content_id));
joinable!(prod_conf_item_content -> prod_conf_item_content_type (conf_item_content_type_id));
joinable!(prod_conf_item_content -> product_config_item (config_item_id));
joinable!(prod_promo_code_contact_mech -> contact_mech (contact_mech_id));
joinable!(prod_promo_code_contact_mech -> product_promo_code (product_promo_code_id));
joinable!(product -> facility (facility_id));
joinable!(product -> geo (origin_geo_id));
joinable!(product -> inventory_item_type (inventory_item_type_id));
joinable!(product -> product_category (primary_product_category_id));
joinable!(product -> product_type (product_type_id));
joinable!(product -> shipment_box_type (default_shipment_box_type_id));
joinable!(product -> uom_type (amount_uom_type_id));
joinable!(product_assoc -> custom_method (estimate_calc_method));
joinable!(product_assoc -> product_assoc_type (product_assoc_type_id));
joinable!(product_assoc -> recurrence_info (recurrence_info_id));
joinable!(product_assoc -> work_effort (routing_work_effort_id));
joinable!(product_attribute -> product (product_id));
joinable!(product_average_cost -> facility (facility_id));
joinable!(product_average_cost -> party (organization_party_id));
joinable!(product_average_cost -> product (product_id));
joinable!(product_average_cost -> product_average_cost_type (product_average_cost_type_id));
joinable!(product_calculated_info -> product (product_id));
joinable!(product_category -> product_category_type (product_category_type_id));
joinable!(product_category_attribute -> product_category (product_category_id));
joinable!(product_category_content -> content (content_id));
joinable!(product_category_content -> product_category (product_category_id));
joinable!(product_category_content -> product_category_content_type (prod_cat_content_type_id));
joinable!(product_category_gl_account -> gl_account (gl_account_id));
joinable!(product_category_gl_account -> gl_account_type (gl_account_type_id));
joinable!(product_category_gl_account -> party (organization_party_id));
joinable!(product_category_gl_account -> product_category (product_category_id));
joinable!(product_category_link -> enumeration (link_type_enum_id));
joinable!(product_category_link -> product_category (product_category_id));
joinable!(product_category_member -> product (product_id));
joinable!(product_category_member -> product_category (product_category_id));
joinable!(product_category_role -> product_category (product_category_id));
joinable!(product_category_type_attr -> product_category_type (product_category_type_id));
joinable!(product_config -> product (product_id));
joinable!(product_config -> product_config_item (config_item_id));
joinable!(product_config_config -> product_config_item (config_item_id));
joinable!(product_config_option -> product_config_item (config_item_id));
joinable!(product_config_product -> product (product_id));
joinable!(product_config_product -> product_config_item (config_item_id));
joinable!(product_config_stats -> product (product_id));
joinable!(product_content -> content (content_id));
joinable!(product_content -> product (product_id));
joinable!(product_content -> product_content_type (product_content_type_id));
joinable!(product_content -> role_type (use_role_type_id));
joinable!(product_content -> uom (use_time_uom_id));
joinable!(product_cost_component_calc -> cost_component_calc (cost_component_calc_id));
joinable!(product_cost_component_calc -> cost_component_type (cost_component_type_id));
joinable!(product_cost_component_calc -> product (product_id));
joinable!(product_facility -> facility (facility_id));
joinable!(product_facility -> product (product_id));
joinable!(product_facility_assoc -> facility_assoc_type (facility_assoc_type_id));
joinable!(product_facility_assoc -> product (product_id));
joinable!(product_facility_location -> product (product_id));
joinable!(product_feature -> product_feature_category (product_feature_category_id));
joinable!(product_feature -> product_feature_type (product_feature_type_id));
joinable!(product_feature -> uom (uom_id));
joinable!(product_feature_appl -> product (product_id));
joinable!(product_feature_appl -> product_feature (product_feature_id));
joinable!(product_feature_appl -> product_feature_appl_type (product_feature_appl_type_id));
joinable!(product_feature_appl_attr -> product (product_id));
joinable!(product_feature_appl_attr -> product_feature (product_feature_id));
joinable!(product_feature_cat_grp_appl -> product_category (product_category_id));
joinable!(product_feature_cat_grp_appl -> product_feature_group (product_feature_group_id));
joinable!(product_feature_category_appl -> product_category (product_category_id));
joinable!(product_feature_category_appl -> product_feature_category (product_feature_category_id));
joinable!(product_feature_data_resource -> data_resource (data_resource_id));
joinable!(product_feature_data_resource -> product_feature (product_feature_id));
joinable!(product_feature_group_appl -> product_feature (product_feature_id));
joinable!(product_feature_group_appl -> product_feature_group (product_feature_group_id));
joinable!(product_feature_iactn -> product_feature_iactn_type (product_feature_iactn_type_id));
joinable!(product_feature_price -> product_price_type (product_price_type_id));
joinable!(product_feature_price -> uom (currency_uom_id));
joinable!(product_geo -> enumeration (product_geo_enum_id));
joinable!(product_geo -> geo (geo_id));
joinable!(product_geo -> product (product_id));
joinable!(product_gl_account -> gl_account (gl_account_id));
joinable!(product_gl_account -> gl_account_type (gl_account_type_id));
joinable!(product_gl_account -> party (organization_party_id));
joinable!(product_gl_account -> product (product_id));
joinable!(product_group_order -> job_sandbox (job_id));
joinable!(product_group_order -> product (product_id));
joinable!(product_group_order -> status_item (status_id));
joinable!(product_keyword_new -> enumeration (keyword_type_id));
joinable!(product_keyword_new -> product (product_id));
joinable!(product_keyword_new -> status_item (status_id));
joinable!(product_maint -> product (product_id));
joinable!(product_maint -> product_maint_type (product_maint_type_id));
joinable!(product_maint -> product_meter_type (interval_meter_type_id));
joinable!(product_maint -> uom (interval_uom_id));
joinable!(product_maint -> work_effort (maint_template_work_effort_id));
joinable!(product_manufacturing_rule -> product_feature (product_feature));
joinable!(product_meter -> product (product_id));
joinable!(product_meter -> product_meter_type (product_meter_type_id));
joinable!(product_meter -> uom (meter_uom_id));
joinable!(product_meter_type -> uom (default_uom_id));
joinable!(product_order_item -> product (product_id));
joinable!(product_payment_method_type -> payment_method_type (payment_method_type_id));
joinable!(product_payment_method_type -> product (product_id));
joinable!(product_payment_method_type -> product_price_purpose (product_price_purpose_id));
joinable!(product_price -> custom_method (custom_price_calc_service));
joinable!(product_price -> geo (tax_auth_geo_id));
joinable!(product_price -> party (tax_auth_party_id));
joinable!(product_price -> product (product_id));
joinable!(product_price -> product_price_purpose (product_price_purpose_id));
joinable!(product_price -> product_price_type (product_price_type_id));
joinable!(product_price -> product_store_group (product_store_group_id));
joinable!(product_price_action -> product_price_action_type (product_price_action_type_id));
joinable!(product_price_action -> product_price_rule (product_price_rule_id));
joinable!(product_price_change -> user_login (changed_by_user_login));
joinable!(product_price_cond -> product_price_rule (product_price_rule_id));
joinable!(product_promo -> party (override_org_party_id));
joinable!(product_promo_action -> custom_method (custom_method_id));
joinable!(product_promo_action -> enumeration (product_promo_action_enum_id));
joinable!(product_promo_action -> order_adjustment_type (order_adjustment_type_id));
joinable!(product_promo_action -> product_promo (product_promo_id));
joinable!(product_promo_category -> enumeration (product_promo_appl_enum_id));
joinable!(product_promo_category -> product_category (product_category_id));
joinable!(product_promo_category -> product_promo (product_promo_id));
joinable!(product_promo_code -> product_promo (product_promo_id));
joinable!(product_promo_code_email -> product_promo_code (product_promo_code_id));
joinable!(product_promo_code_party -> party (party_id));
joinable!(product_promo_code_party -> product_promo_code (product_promo_code_id));
joinable!(product_promo_cond -> custom_method (custom_method_id));
joinable!(product_promo_cond -> product_promo (product_promo_id));
joinable!(product_promo_content -> content (content_id));
joinable!(product_promo_content -> product_content_type (product_promo_content_type_id));
joinable!(product_promo_content -> product_promo (product_promo_id));
joinable!(product_promo_product -> enumeration (product_promo_appl_enum_id));
joinable!(product_promo_product -> product (product_id));
joinable!(product_promo_product -> product_promo (product_promo_id));
joinable!(product_promo_rule -> product_promo (product_promo_id));
joinable!(product_promo_use -> order_header (order_id));
joinable!(product_promo_use -> party (party_id));
joinable!(product_promo_use -> product_promo (product_promo_id));
joinable!(product_promo_use -> product_promo_code (product_promo_code_id));
joinable!(product_review -> product (product_id));
joinable!(product_review -> product_store (product_store_id));
joinable!(product_review -> status_item (status_id));
joinable!(product_review -> user_login (user_login_id));
joinable!(product_role -> product (product_id));
joinable!(product_search_constraint -> product_search_result (product_search_result_id));
joinable!(product_store -> facility (inventory_facility_id));
joinable!(product_store -> party (pay_to_party_id));
joinable!(product_store -> product_store_group (primary_store_group_id));
joinable!(product_store -> uom (default_currency_uom_id));
joinable!(product_store_catalog -> prod_catalog (prod_catalog_id));
joinable!(product_store_catalog -> product_store (product_store_id));
joinable!(product_store_email_setting -> enumeration (email_type));
joinable!(product_store_email_setting -> product_store (product_store_id));
joinable!(product_store_facility -> facility (facility_id));
joinable!(product_store_facility -> product_store (product_store_id));
joinable!(product_store_fin_act_setting -> enumeration (replenish_method_enum_id));
joinable!(product_store_fin_act_setting -> fin_account_type (fin_account_type_id));
joinable!(product_store_fin_act_setting -> product_store (product_store_id));
joinable!(product_store_fin_act_setting -> survey (purchase_survey_id));
joinable!(product_store_group -> product_store_group_type (product_store_group_type_id));
joinable!(product_store_group_member -> product_store (product_store_id));
joinable!(product_store_group_member -> product_store_group (product_store_group_id));
joinable!(product_store_group_role -> product_store_group (product_store_group_id));
joinable!(product_store_keyword_ovrd -> enumeration (target_type_enum_id));
joinable!(product_store_keyword_ovrd -> product_store (product_store_id));
joinable!(product_store_payment_setting -> custom_method (payment_custom_method_id));
joinable!(product_store_payment_setting -> enumeration (payment_service_type_enum_id));
joinable!(product_store_payment_setting -> payment_gateway_config (payment_gateway_config_id));
joinable!(product_store_payment_setting -> payment_method_type (payment_method_type_id));
joinable!(product_store_payment_setting -> product_store (product_store_id));
joinable!(product_store_promo_appl -> product_promo (product_promo_id));
joinable!(product_store_promo_appl -> product_store (product_store_id));
joinable!(product_store_role -> product_store (product_store_id));
joinable!(product_store_shipment_meth -> custom_method (shipment_custom_method_id));
joinable!(product_store_shipment_meth -> shipment_gateway_config (shipment_gateway_config_id));
joinable!(product_store_shipment_meth -> shipment_method_type (shipment_method_type_id));
joinable!(product_store_survey_appl -> product_store (product_store_id));
joinable!(product_store_survey_appl -> survey (survey_id));
joinable!(product_store_survey_appl -> survey_appl_type (survey_appl_type_id));
joinable!(product_store_telecom_setting -> custom_method (telecom_custom_method_id));
joinable!(product_store_telecom_setting -> enumeration (telecom_msg_type_enum_id));
joinable!(product_store_telecom_setting -> product_store (product_store_id));
joinable!(product_store_telecom_setting -> telecom_gateway_config (telecom_gateway_config_id));
joinable!(product_store_telecom_setting -> telecom_method_type (telecom_method_type_id));
joinable!(product_store_vendor_payment -> enumeration (credit_card_enum_id));
joinable!(product_store_vendor_payment -> party (vendor_party_id));
joinable!(product_store_vendor_payment -> payment_method_type (payment_method_type_id));
joinable!(product_store_vendor_payment -> product_store (product_store_id));
joinable!(product_store_vendor_shipment -> product_store (product_store_id));
joinable!(product_store_vendor_shipment -> shipment_method_type (shipment_method_type_id));
joinable!(product_subscription_resource -> product (product_id));
joinable!(product_subscription_resource -> role_type (use_role_type_id));
joinable!(product_subscription_resource -> subscription_resource (subscription_resource_id));
joinable!(product_type_attr -> product_type (product_type_id));
joinable!(protected_view -> security_group (group_id));
joinable!(quantity_break -> quantity_break_type (quantity_break_type_id));
joinable!(quote -> enumeration (sales_channel_enum_id));
joinable!(quote -> party (party_id));
joinable!(quote -> product_store (product_store_id));
joinable!(quote -> quote_type (quote_type_id));
joinable!(quote -> status_item (status_id));
joinable!(quote -> uom (currency_uom_id));
joinable!(quote_adjustment -> gl_account (override_gl_account_id));
joinable!(quote_adjustment -> order_adjustment_type (quote_adjustment_type_id));
joinable!(quote_adjustment -> product_promo (product_promo_id));
joinable!(quote_adjustment -> quote (quote_id));
joinable!(quote_adjustment -> user_login (created_by_user_login));
joinable!(quote_attribute -> quote (quote_id));
joinable!(quote_coefficient -> quote (quote_id));
joinable!(quote_item -> cust_request (cust_request_id));
joinable!(quote_item -> deliverable_type (deliverable_type_id));
joinable!(quote_item -> product (product_id));
joinable!(quote_item -> product_feature (product_feature_id));
joinable!(quote_item -> quote (quote_id));
joinable!(quote_item -> skill_type (skill_type_id));
joinable!(quote_item -> uom (uom_id));
joinable!(quote_item -> work_effort (work_effort_id));
joinable!(quote_note -> note_data (note_id));
joinable!(quote_note -> quote (quote_id));
joinable!(quote_role -> party (party_id));
joinable!(quote_role -> quote (quote_id));
joinable!(quote_term -> quote (quote_id));
joinable!(quote_term -> term_type (term_type_id));
joinable!(quote_type_attr -> quote_type (quote_type_id));
joinable!(quote_work_effort -> quote (quote_id));
joinable!(quote_work_effort -> work_effort (work_effort_id));
joinable!(rate_amount -> empl_position_type (empl_position_type_id));
joinable!(rate_amount -> party (party_id));
joinable!(rate_amount -> period_type (period_type_id));
joinable!(rate_amount -> rate_type (rate_type_id));
joinable!(rate_amount -> uom (rate_currency_uom_id));
joinable!(rate_amount -> work_effort (work_effort_id));
joinable!(reorder_guideline -> facility (facility_id));
joinable!(reorder_guideline -> geo (geo_id));
joinable!(reorder_guideline -> party (party_id));
joinable!(reorder_guideline -> product (product_id));
joinable!(requirement -> deliverable (deliverable_id));
joinable!(requirement -> facility (facility_id));
joinable!(requirement -> fixed_asset (fixed_asset_id));
joinable!(requirement -> product (product_id));
joinable!(requirement -> requirement_type (requirement_type_id));
joinable!(requirement -> status_item (status_id));
joinable!(requirement_attribute -> requirement (requirement_id));
joinable!(requirement_budget_allocation -> requirement (requirement_id));
joinable!(requirement_cust_request -> requirement (requirement_id));
joinable!(requirement_role -> party (party_id));
joinable!(requirement_role -> requirement (requirement_id));
joinable!(requirement_status -> requirement (requirement_id));
joinable!(requirement_status -> status_item (status_id));
joinable!(requirement_status -> user_login (change_by_user_login_id));
joinable!(requirement_type_attr -> requirement_type (requirement_type_id));
joinable!(responding_party -> contact_mech (contact_mech_id));
joinable!(responding_party -> cust_request (cust_request_id));
joinable!(responding_party -> party (party_id));
joinable!(return_adjustment -> gl_account (override_gl_account_id));
joinable!(return_adjustment -> order_adjustment (order_adjustment_id));
joinable!(return_adjustment -> product_promo (product_promo_id));
joinable!(return_adjustment -> return_adjustment_type (return_adjustment_type_id));
joinable!(return_adjustment -> return_header (return_id));
joinable!(return_adjustment -> return_type (return_type_id));
joinable!(return_adjustment -> tax_authority_rate_product (tax_authority_rate_seq_id));
joinable!(return_adjustment -> user_login (created_by_user_login));
joinable!(return_contact_mech -> contact_mech (contact_mech_id));
joinable!(return_contact_mech -> contact_mech_purpose_type (contact_mech_purpose_type_id));
joinable!(return_contact_mech -> return_header (return_id));
joinable!(return_header -> billing_account (billing_account_id));
joinable!(return_header -> contact_mech (origin_contact_mech_id));
joinable!(return_header -> facility (destination_facility_id));
joinable!(return_header -> fin_account (fin_account_id));
joinable!(return_header -> payment_method (payment_method_id));
joinable!(return_header -> return_header_type (return_header_type_id));
joinable!(return_header -> status_item (status_id));
joinable!(return_header -> uom (currency_uom_id));
joinable!(return_item -> order_header (order_id));
joinable!(return_item -> product (product_id));
joinable!(return_item -> return_header (return_id));
joinable!(return_item -> return_item_response (return_item_response_id));
joinable!(return_item -> return_item_type (return_item_type_id));
joinable!(return_item -> return_reason (return_reason_id));
joinable!(return_item -> return_type (return_type_id));
joinable!(return_item_billing -> return_header (return_id));
joinable!(return_item_billing -> shipment_receipt (shipment_receipt_id));
joinable!(return_item_response -> billing_account (billing_account_id));
joinable!(return_item_response -> fin_account_trans (fin_account_trans_id));
joinable!(return_item_response -> order_header (replacement_order_id));
joinable!(return_item_response -> order_payment_preference (order_payment_preference_id));
joinable!(return_item_response -> payment (payment_id));
joinable!(return_item_shipment -> return_header (return_id));
joinable!(return_item_shipment -> shipment (shipment_id));
joinable!(return_item_type_map -> return_header_type (return_header_type_id));
joinable!(return_status -> return_header (return_id));
joinable!(return_status -> status_item (status_id));
joinable!(return_status -> user_login (change_by_user_login_id));
joinable!(role_type_attr -> role_type (role_type_id));
joinable!(salary_step_new -> pay_grade (pay_grade_id));
joinable!(sales_forecast -> custom_time_period (custom_time_period_id));
joinable!(sales_forecast -> uom (currency_uom_id));
joinable!(sales_forecast_detail -> product (product_id));
joinable!(sales_forecast_detail -> product_category (product_category_id));
joinable!(sales_forecast_detail -> sales_forecast (sales_forecast_id));
joinable!(sales_forecast_detail -> uom (quantity_uom_id));
joinable!(sales_forecast_history -> custom_time_period (custom_time_period_id));
joinable!(sales_forecast_history -> sales_forecast (sales_forecast_id));
joinable!(sales_forecast_history -> uom (currency_uom_id));
joinable!(sales_forecast_history -> user_login (modified_by_user_login_id));
joinable!(sales_opportunity -> data_source (data_source_id));
joinable!(sales_opportunity -> enumeration (type_enum_id));
joinable!(sales_opportunity -> marketing_campaign (marketing_campaign_id));
joinable!(sales_opportunity -> sales_opportunity_stage (opportunity_stage_id));
joinable!(sales_opportunity -> uom (currency_uom_id));
joinable!(sales_opportunity -> user_login (created_by_user_login));
joinable!(sales_opportunity_competitor -> sales_opportunity (sales_opportunity_id));
joinable!(sales_opportunity_history -> sales_opportunity (sales_opportunity_id));
joinable!(sales_opportunity_history -> sales_opportunity_stage (opportunity_stage_id));
joinable!(sales_opportunity_history -> uom (currency_uom_id));
joinable!(sales_opportunity_history -> user_login (modified_by_user_login));
joinable!(sales_opportunity_quote -> quote (quote_id));
joinable!(sales_opportunity_quote -> sales_opportunity (sales_opportunity_id));
joinable!(sales_opportunity_role -> party (party_id));
joinable!(sales_opportunity_role -> role_type (role_type_id));
joinable!(sales_opportunity_role -> sales_opportunity (sales_opportunity_id));
joinable!(sales_opportunity_trck_code -> sales_opportunity (sales_opportunity_id));
joinable!(sales_opportunity_work_effort -> sales_opportunity (sales_opportunity_id));
joinable!(sales_opportunity_work_effort -> work_effort (work_effort_id));
joinable!(security_group_permission -> security_group (group_id));
joinable!(segment_group -> product_store (product_store_id));
joinable!(segment_group -> segment_group_type (segment_group_type_id));
joinable!(segment_group_classification -> party_classification_group (party_classification_group_id));
joinable!(segment_group_classification -> segment_group (segment_group_id));
joinable!(segment_group_geo -> geo (geo_id));
joinable!(segment_group_geo -> segment_group (segment_group_id));
joinable!(segment_group_role -> segment_group (segment_group_id));
joinable!(server_hit -> server_hit_type (hit_type_id));
joinable!(server_hit -> visit (visit_id));
joinable!(server_hit_bin -> server_hit_type (hit_type_id));
joinable!(shipment -> order_header (primary_order_id));
joinable!(shipment -> picklist_bin (picklist_bin_id));
joinable!(shipment -> return_header (primary_return_id));
joinable!(shipment -> shipment_type (shipment_type_id));
joinable!(shipment -> status_item (status_id));
joinable!(shipment -> uom (currency_uom_id));
joinable!(shipment_attribute -> shipment (shipment_id));
joinable!(shipment_contact_mech -> contact_mech (contact_mech_id));
joinable!(shipment_contact_mech -> shipment (shipment_id));
joinable!(shipment_contact_mech -> shipment_contact_mech_type (shipment_contact_mech_type_id));
joinable!(shipment_cost_estimate -> party (party_id));
joinable!(shipment_cost_estimate -> product_store_shipment_meth (product_store_ship_meth_id));
joinable!(shipment_cost_estimate -> role_type (role_type_id));
joinable!(shipment_gateway_config -> shipment_gateway_config_type (shipment_gateway_conf_type_id));
joinable!(shipment_gateway_dhl -> shipment_gateway_config (shipment_gateway_config_id));
joinable!(shipment_gateway_fedex -> shipment_gateway_config (shipment_gateway_config_id));
joinable!(shipment_gateway_ups -> shipment_gateway_config (shipment_gateway_config_id));
joinable!(shipment_gateway_usps -> shipment_gateway_config (shipment_gateway_config_id));
joinable!(shipment_item -> product (product_id));
joinable!(shipment_item -> shipment (shipment_id));
joinable!(shipment_item_feature -> product_feature (product_feature_id));
joinable!(shipment_package -> shipment (shipment_id));
joinable!(shipment_package -> shipment_box_type (shipment_box_type_id));
joinable!(shipment_package_content -> product (sub_product_id));
joinable!(shipment_package_route_seg -> uom (currency_uom_id));
joinable!(shipment_receipt -> inventory_item (inventory_item_id));
joinable!(shipment_receipt -> product (product_id));
joinable!(shipment_receipt -> rejection_reason (rejection_id));
joinable!(shipment_receipt -> user_login (received_by_user_login_id));
joinable!(shipment_receipt_role -> party (party_id));
joinable!(shipment_receipt_role -> shipment_receipt (receipt_id));
joinable!(shipment_route_segment -> delivery (delivery_id));
joinable!(shipment_route_segment -> party (carrier_party_id));
joinable!(shipment_route_segment -> shipment (shipment_id));
joinable!(shipment_route_segment -> shipment_method_type (shipment_method_type_id));
joinable!(shipment_route_segment -> status_item (carrier_service_status_id));
joinable!(shipment_status -> shipment (shipment_id));
joinable!(shipment_status -> status_item (status_id));
joinable!(shipment_status -> user_login (change_by_user_login_id));
joinable!(shipment_time_estimate -> uom (lead_time_uom_id));
joinable!(shipment_type_attr -> shipment_type (shipment_type_id));
joinable!(shipping_document -> document (document_id));
joinable!(shopping_list -> contact_mech (contact_mech_id));
joinable!(shopping_list -> party (party_id));
joinable!(shopping_list -> payment_method (payment_method_id));
joinable!(shopping_list -> product_promo_code (product_promo_code_id));
joinable!(shopping_list -> product_store (product_store_id));
joinable!(shopping_list -> recurrence_info (recurrence_info_id));
joinable!(shopping_list -> shopping_list_type (shopping_list_type_id));
joinable!(shopping_list_item -> product (product_id));
joinable!(shopping_list_item -> shopping_list (shopping_list_id));
joinable!(shopping_list_item_survey -> shopping_list (shopping_list_id));
joinable!(shopping_list_item_survey -> survey_response (survey_response_id));
joinable!(shopping_list_work_effort -> shopping_list (shopping_list_id));
joinable!(shopping_list_work_effort -> work_effort (work_effort_id));
joinable!(status_item -> status_type (status_type_id));
joinable!(subscription -> contact_mech (contact_mech_id));
joinable!(subscription -> inventory_item (inventory_item_id));
joinable!(subscription -> need_type (need_type_id));
joinable!(subscription -> product (product_id));
joinable!(subscription -> product_category (product_category_id));
joinable!(subscription -> subscription_resource (subscription_resource_id));
joinable!(subscription -> subscription_type (subscription_type_id));
joinable!(subscription_attribute -> subscription (subscription_id));
joinable!(subscription_comm_event -> communication_event (communication_event_id));
joinable!(subscription_comm_event -> subscription (subscription_id));
joinable!(subscription_fulfillment_piece -> subscription (subscription_id));
joinable!(subscription_fulfillment_piece -> subscription_activity (subscription_activity_id));
joinable!(subscription_resource -> content (content_id));
joinable!(subscription_resource -> web_site (web_site_id));
joinable!(subscription_type_attr -> subscription_type (subscription_type_id));
joinable!(supplier_product -> party (party_id));
joinable!(supplier_product -> product (product_id));
joinable!(supplier_product -> supplier_pref_order (supplier_pref_order_id));
joinable!(supplier_product -> supplier_rating_type (supplier_rating_type_id));
joinable!(supplier_product_feature -> party (party_id));
joinable!(supplier_product_feature -> product_feature (product_feature_id));
joinable!(supplier_product_feature -> uom (uom_id));
joinable!(survey_multi_resp -> survey (survey_id));
joinable!(survey_page -> survey (survey_id));
joinable!(survey_question -> geo (geo_id));
joinable!(survey_question -> survey_question_category (survey_question_category_id));
joinable!(survey_question -> survey_question_type (survey_question_type_id));
joinable!(survey_question_appl -> survey (survey_id));
joinable!(survey_question_appl -> survey_question (survey_question_id));
joinable!(survey_question_option -> survey_question (survey_question_id));
joinable!(survey_response -> status_item (status_id));
joinable!(survey_response -> survey (survey_id));
joinable!(survey_response_answer -> content (content_id));
joinable!(survey_response_answer -> survey_question (survey_question_id));
joinable!(survey_response_answer -> survey_response (survey_response_id));
joinable!(survey_trigger -> survey (survey_id));
joinable!(survey_trigger -> survey_appl_type (survey_appl_type_id));
joinable!(tax_authority -> geo (tax_auth_geo_id));
joinable!(tax_authority -> party (tax_auth_party_id));
joinable!(tax_authority_assoc -> tax_authority_assoc_type (tax_authority_assoc_type_id));
joinable!(tax_authority_category -> product_category (product_category_id));
joinable!(tax_authority_gl_account -> gl_account (gl_account_id));
joinable!(tax_authority_gl_account -> party (organization_party_id));
joinable!(tax_authority_rate_product -> product_category (product_category_id));
joinable!(tax_authority_rate_product -> product_store (product_store_id));
joinable!(tax_authority_rate_product -> tax_authority_rate_type (tax_authority_rate_type_id));
joinable!(tech_data_calendar -> tech_data_calendar_week (calendar_week_id));
joinable!(tech_data_calendar_exc_day -> tech_data_calendar (calendar_id));
joinable!(tech_data_calendar_exc_week -> tech_data_calendar (calendar_id));
joinable!(tech_data_calendar_exc_week -> tech_data_calendar_week (calendar_week_id));
joinable!(telecom_number -> contact_mech (contact_mech_id));
joinable!(term_type_attr -> term_type (term_type_id));
joinable!(testing -> testing_type (testing_type_id));
joinable!(testing_item -> testing (testing_id));
joinable!(testing_node_member -> testing (testing_id));
joinable!(testing_node_member -> testing_node (testing_node_id));
joinable!(testing_status -> status_item (status_id));
joinable!(testing_status -> user_login (change_by_user_login_id));
joinable!(third_party_login -> product_store (product_store_id));
joinable!(time_entry -> party (party_id));
joinable!(time_entry -> rate_type (rate_type_id));
joinable!(time_entry -> timesheet (timesheet_id));
joinable!(time_entry -> work_effort (work_effort_id));
joinable!(timesheet -> status_item (status_id));
joinable!(timesheet -> user_login (approved_by_user_login_id));
joinable!(timesheet_role -> party (party_id));
joinable!(timesheet_role -> timesheet (timesheet_id));
joinable!(tracking_code -> marketing_campaign (marketing_campaign_id));
joinable!(tracking_code -> tracking_code_type (tracking_code_type_id));
joinable!(tracking_code_order -> order_header (order_id));
joinable!(tracking_code_order -> tracking_code (tracking_code_id));
joinable!(tracking_code_order -> tracking_code_type (tracking_code_type_id));
joinable!(tracking_code_order_return -> order_header (order_id));
joinable!(tracking_code_order_return -> return_header (return_id));
joinable!(tracking_code_order_return -> tracking_code (tracking_code_id));
joinable!(tracking_code_order_return -> tracking_code_type (tracking_code_type_id));
joinable!(tracking_code_visit -> enumeration (source_enum_id));
joinable!(tracking_code_visit -> tracking_code (tracking_code_id));
joinable!(uom -> uom_type (uom_type_id));
joinable!(uom_conversion -> custom_method (custom_method_id));
joinable!(uom_conversion_dated -> custom_method (custom_method_id));
joinable!(uom_conversion_dated -> enumeration (purpose_enum_id));
joinable!(uom_group -> uom (uom_id));
joinable!(user_agent -> browser_type (browser_type_id));
joinable!(user_agent -> platform_type (platform_type_id));
joinable!(user_agent -> protocol_type (protocol_type_id));
joinable!(user_agent -> user_agent_method_type (user_agent_method_type_id));
joinable!(user_agent -> user_agent_type (user_agent_type_id));
joinable!(user_login_history -> party (party_id));
joinable!(user_login_history -> user_login (user_login_id));
joinable!(user_login_password_history -> user_login (user_login_id));
joinable!(user_login_security_group -> security_group (group_id));
joinable!(user_login_security_group -> user_login (user_login_id));
joinable!(user_login_session -> user_login (user_login_id));
joinable!(user_preference -> user_pref_group_type (user_pref_group_type_id));
joinable!(valid_contact_mech_role -> contact_mech_type (contact_mech_type_id));
joinable!(valid_contact_mech_role -> role_type (role_type_id));
joinable!(valid_responsibility -> empl_position_type (empl_position_type_id));
joinable!(valid_responsibility -> responsibility_type (responsibility_type_id));
joinable!(variance_reason_gl_account -> gl_account (gl_account_id));
joinable!(variance_reason_gl_account -> party (organization_party_id));
joinable!(variance_reason_gl_account -> variance_reason (variance_reason_id));
joinable!(vendor -> party (party_id));
joinable!(vendor_product -> party (vendor_party_id));
joinable!(vendor_product -> product (product_id));
joinable!(vendor_product -> product_store_group (product_store_group_id));
joinable!(video_data_resource -> data_resource (data_resource_id));
joinable!(visit -> user_agent (user_agent_id));
joinable!(visit -> visitor (visitor_id));
joinable!(visual_theme -> visual_theme_set (visual_theme_set_id));
joinable!(visual_theme_resource -> enumeration (resource_type_enum_id));
joinable!(visual_theme_resource -> visual_theme (visual_theme_id));
joinable!(web_page -> content (content_id));
joinable!(web_page -> web_site (web_site_id));
joinable!(web_site -> product_store (product_store_id));
joinable!(web_site -> visual_theme_set (visual_theme_set_id));
joinable!(web_site_contact_list -> contact_list (contact_list_id));
joinable!(web_site_contact_list -> web_site (web_site_id));
joinable!(web_site_content -> content (content_id));
joinable!(web_site_content -> web_site (web_site_id));
joinable!(web_site_content -> web_site_content_type (web_site_content_type_id));
joinable!(web_site_path_alias -> content (content_id));
joinable!(web_site_path_alias -> web_site (web_site_id));
joinable!(web_site_publish_point -> content (content_id));
joinable!(web_site_role -> web_site (web_site_id));
joinable!(web_user_preference -> party (party_id));
joinable!(web_user_preference -> user_login (user_login_id));
joinable!(web_user_preference -> web_preference_type (web_preference_type_id));
joinable!(work_effort -> accommodation_map (accommodation_map_id));
joinable!(work_effort -> accommodation_spot (accommodation_spot_id));
joinable!(work_effort -> custom_method (estimate_calc_method));
joinable!(work_effort -> enumeration (scope_enum_id));
joinable!(work_effort -> facility (facility_id));
joinable!(work_effort -> fixed_asset (fixed_asset_id));
joinable!(work_effort -> note_data (note_id));
joinable!(work_effort -> recurrence_info (recurrence_info_id));
joinable!(work_effort -> runtime_data (runtime_data_id));
joinable!(work_effort -> status_item (current_status_id));
joinable!(work_effort -> temporal_expression (temp_expr_id));
joinable!(work_effort -> uom (money_uom_id));
joinable!(work_effort -> work_effort_purpose_type (work_effort_purpose_type_id));
joinable!(work_effort -> work_effort_type (work_effort_type_id));
joinable!(work_effort_assoc -> work_effort_assoc_type (work_effort_assoc_type_id));
joinable!(work_effort_assoc_type_attr -> work_effort_assoc_type (work_effort_assoc_type_id));
joinable!(work_effort_attribute -> work_effort (work_effort_id));
joinable!(work_effort_billing -> work_effort (work_effort_id));
joinable!(work_effort_contact_mech_new -> contact_mech (contact_mech_id));
joinable!(work_effort_contact_mech_new -> work_effort (work_effort_id));
joinable!(work_effort_content -> content (content_id));
joinable!(work_effort_content -> work_effort (work_effort_id));
joinable!(work_effort_content -> work_effort_content_type (work_effort_content_type_id));
joinable!(work_effort_cost_calc -> cost_component_calc (cost_component_calc_id));
joinable!(work_effort_cost_calc -> cost_component_type (cost_component_type_id));
joinable!(work_effort_cost_calc -> work_effort (work_effort_id));
joinable!(work_effort_deliverable_prod -> deliverable (deliverable_id));
joinable!(work_effort_deliverable_prod -> work_effort (work_effort_id));
joinable!(work_effort_event_reminder -> contact_mech (contact_mech_id));
joinable!(work_effort_event_reminder -> party (party_id));
joinable!(work_effort_event_reminder -> work_effort (work_effort_id));
joinable!(work_effort_fixed_asset_assign -> fixed_asset (fixed_asset_id));
joinable!(work_effort_fixed_asset_assign -> work_effort (work_effort_id));
joinable!(work_effort_fixed_asset_std -> fixed_asset_type (fixed_asset_type_id));
joinable!(work_effort_fixed_asset_std -> work_effort (work_effort_id));
joinable!(work_effort_good_standard -> product (product_id));
joinable!(work_effort_good_standard -> status_item (status_id));
joinable!(work_effort_good_standard -> work_effort (work_effort_id));
joinable!(work_effort_good_standard -> work_effort_good_standard_type (work_effort_good_std_type_id));
joinable!(work_effort_ical_data -> work_effort (work_effort_id));
joinable!(work_effort_inventory_assign -> inventory_item (inventory_item_id));
joinable!(work_effort_inventory_assign -> status_item (status_id));
joinable!(work_effort_inventory_assign -> work_effort (work_effort_id));
joinable!(work_effort_inventory_produced -> inventory_item (inventory_item_id));
joinable!(work_effort_inventory_produced -> work_effort (work_effort_id));
joinable!(work_effort_keyword -> work_effort (work_effort_id));
joinable!(work_effort_note -> note_data (note_id));
joinable!(work_effort_note -> work_effort (work_effort_id));
joinable!(work_effort_party_assignment -> facility (facility_id));
joinable!(work_effort_party_assignment -> user_login (assigned_by_user_login_id));
joinable!(work_effort_party_assignment -> work_effort (work_effort_id));
joinable!(work_effort_review -> status_item (status_id));
joinable!(work_effort_review -> user_login (user_login_id));
joinable!(work_effort_review -> work_effort (work_effort_id));
joinable!(work_effort_search_constraint -> work_effort_search_result (work_effort_search_result_id));
joinable!(work_effort_skill_standard -> skill_type (skill_type_id));
joinable!(work_effort_skill_standard -> work_effort (work_effort_id));
joinable!(work_effort_status -> status_item (status_id));
joinable!(work_effort_status -> user_login (set_by_user_login));
joinable!(work_effort_status -> work_effort (work_effort_id));
joinable!(work_effort_survey_appl -> product_store_survey_appl (survey_id));
joinable!(work_effort_survey_appl -> survey (survey_id));
joinable!(work_effort_survey_appl -> work_effort (work_effort_id));
joinable!(work_effort_trans_box -> work_effort (process_work_effort_id));
joinable!(work_effort_type_attr -> work_effort_type (work_effort_type_id));
joinable!(work_order_item_fulfillment -> order_header (order_id));
joinable!(work_order_item_fulfillment -> work_effort (work_effort_id));
joinable!(work_requirement_fulfillment -> requirement (requirement_id));
joinable!(work_requirement_fulfillment -> work_effort (work_effort_id));
joinable!(work_requirement_fulfillment -> work_req_fulf_type (work_req_fulf_type_id));

allow_tables_to_appear_in_same_query!(
    accommodation_class,
    accommodation_map,
    accommodation_map_type,
    accommodation_spot,
);
