// @generated
pub mod sf {
    pub mod solana {
        pub mod r#type {
            // @@protoc_insertion_point(attribute:sf.solana.type.v1)
            pub mod v1 {
                include!("sf.solana.type.v1.rs");
                // @@protoc_insertion_point(sf.solana.type.v1)
            }
        }
    }
}
pub mod sol {
    pub mod block {
        // @@protoc_insertion_point(attribute:sol.block.v1)
        pub mod v1 {
            include!("sol.block.v1.rs");
            // @@protoc_insertion_point(sol.block.v1)
        }
    }
    pub mod transactions {
        pub mod journal {
            // @@protoc_insertion_point(attribute:sol.transactions.journal.v1)
            pub mod v1 {
                include!("sol.transactions.journal.v1.rs");
                // @@protoc_insertion_point(sol.transactions.journal.v1)
            }
        }
        // @@protoc_insertion_point(attribute:sol.transactions.v1)
        pub mod v1 {
            include!("sol.transactions.v1.rs");
            // @@protoc_insertion_point(sol.transactions.v1)
        }
        pub mod vault {
            // @@protoc_insertion_point(attribute:sol.transactions.vault.v1)
            pub mod v1 {
                include!("sol.transactions.vault.v1.rs");
                // @@protoc_insertion_point(sol.transactions.vault.v1)
            }
        }
    }
}
