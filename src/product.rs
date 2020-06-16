// Copyright (c) 2020 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

enumerate_id_set! {
    /// Visual Studio product IDs.
    pub enum Product {
        /// Visual Studio Enterprise.
        Enterprise = "Microsoft.VisualStudio.Product.Enterprise",
        /// Visual Studio Professional.
        Professional = "Microsoft.VisualStudio.Product.Professional",
        /// Visual Studio Community.
        Community = "Microsoft.VisualStudio.Product.Community",
        /// Visual Studio Team Explorer.
        TeamExplorer = "Microsoft.VisualStudio.Product.TeamExplorer",
        /// Visual Studio Desktop Express.
        WDExpress = "Microsoft.VisualStudio.Product.WDExpress",
        /// Visual Studio Build Tools.
        BuildTools = "Microsoft.VisualStudio.Product.BuildTools",
        /// Visual Studio Test Agent.
        TestAgent = "Microsoft.VisualStudio.Product.TestAgent",
        /// Visual Studio Test Controller.
        TestController = "Microsoft.VisualStudio.Product.TestController",
        /// Visual Studio Test Professional.
        TestProfessional = "Microsoft.VisualStudio.Product.TestProfessional",
        /// Visual Studio Feedback Client.
        FeedbackClient = "Microsoft.VisualStudio.Product.FeedbackClient",
    }
}
