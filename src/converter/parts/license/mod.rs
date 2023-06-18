use std::{collections::HashMap, fmt::Display, fs, str::FromStr};

use serde_json::json;
use strum::{EnumIter, IntoEnumIterator};

use crate::{
    converter::{Repository, RepositoryPlatform},
    utils::{paths, GenMarkdown},
};

#[derive(Debug, Clone, PartialEq, EnumIter, Copy, Eq, Hash)]
/// The available licenses for a project which a user can choose from
pub enum SupportedLicense {
    Unknown,
    Apache20,
    MIT,
    GNUGeneralPublicLicenseV30,
    BSD2Clause,
    BSD3Clause,
    GNULesserGeneralPublicLicenseV30,
    MozillaPublicLicense20,
    GNUAfferoGeneralPublicLicenseV30,
    GNUGeneralPublicLicenseV20,
    EclipsePublicLicense20,
    Unlicense,
    CreativeCommonsZeroV10Universal,
    GNUGeneralPublicLicenseV20OrLater,
    GNUAfferoGeneralPublicLicenseV10,
    GNULesserGeneralPublicLicenseV21,
    GNULesserGeneralPublicLicenseV20OrLater,
    ISC,
    MicrosoftPublicLicense,
    MicrosoftReciprocalLicense,
    GNUAfferoGeneralPublicLicenseV30OrLater,
    EuropeanUnionPublicLicense11,
    DoWhatTheFuckYouWantToPublicLicense,
    ZlibLicense,
    GNUAfferoGeneralPublicLicenseV20OrLaterWithAutoconfException,
    TheUnlicense,
    GNULesserGeneralPublicLicenseV21OrLater,
    GNULesserGeneralPublicLicenseV20,
    BoostSoftwareLicense10,
    GNUGeneralPublicLicenseV30Only,
    GNULesserGeneralPublicLicenseV30Only,
    BSD3ClauseClear,
    BSD4Clause,
    GNUGeneralPublicLicenseV30OrLaterWithAutoconfException,
    EuropeanUnionPublicLicense12,
    BSD3ClauseOriginal,
    SILOpenFontLicense11,
    GNUGeneralPublicLicenseV10,
    PostgreSQLLicense,
    ArtisticLicense20,
    ISCLicense,
    BSD2ClauseFreeBSD,
    BSD3ClauseNew,
    BSD3ClauseMultiUse,
    ApacheLicense20WithGCCException,
    GNUGeneralPublicLicenseV10OrLater,
    BSD3ClauseLBNL,
    BSD3ClauseClearNew,
    BSD3ClauseNoNuclearLicense,
    BSD3ClauseOpenSSL,
    BSD3ClauseAttribution,
    BSD4ClauseUC,
    GNUGeneralPublicLicenseV20OrLaterWithBisonException,
    GNULesserGeneralPublicLicenseV21OrLaterWithGCCException,
    ZlibLicenseOnly,
    BSD3ClauseLimited,
    BSD3ClauseRuby,
    BSD3ClauseUniversityOfIllinois,
    GNULesserGeneralPublicLicenseV21OrLaterWithClasspathException,
    BSD3ClauseUCBerkeley,
    MicrosoftPublicLicense20,
    ApacheLicense11,
    BSD3ClauseRevised,
    GNUGeneralPublicLicenseV20WithAutoconfException,
    ApacheLicense20WithLLVMException,
    BlueOakModelLicense100,
    CC010Universal,
    CreativeCommonsAttributionShareAlike40International,
}

impl ToString for SupportedLicense {
    fn to_string(&self) -> String {
        match self {
            SupportedLicense::Unknown => "Unknown",
            SupportedLicense::Apache20 => "Apache-2.0",
            SupportedLicense::MIT => "MIT",
            SupportedLicense::GNUGeneralPublicLicenseV30 => "GNU General Public License v3.0",
            SupportedLicense::BSD2Clause => "BSD 2-Clause",
            SupportedLicense::BSD3Clause => "BSD 3-Clause",
            SupportedLicense::GNULesserGeneralPublicLicenseV30 => {
                "GNU Lesser General Public License v3.0"
            }
            SupportedLicense::MozillaPublicLicense20 => "Mozilla Public License 2.0",
            SupportedLicense::GNUAfferoGeneralPublicLicenseV30 => {
                "GNU Affero General Public License v3.0"
            }
            SupportedLicense::GNUGeneralPublicLicenseV20 => "GNU General Public License v2.0",
            SupportedLicense::EclipsePublicLicense20 => "Eclipse Public License 2.0",
            SupportedLicense::Unlicense => "Unlicense",
            SupportedLicense::CreativeCommonsZeroV10Universal => {
                "Creative Commons Zero v1.0 Universal"
            }
            SupportedLicense::GNUGeneralPublicLicenseV20OrLater => {
                "GNU General Public License v2.0 or later"
            }
            SupportedLicense::GNUAfferoGeneralPublicLicenseV10 => {
                "GNU Affero General Public License v1.0"
            }
            SupportedLicense::GNULesserGeneralPublicLicenseV21 => {
                "GNU Lesser General Public License v2.1"
            }
            SupportedLicense::GNULesserGeneralPublicLicenseV20OrLater => {
                "GNU Lesser General Public License v2.0 or later"
            }
            SupportedLicense::ISC => "ISC",
            SupportedLicense::MicrosoftPublicLicense => "Microsoft Public License",
            SupportedLicense::MicrosoftReciprocalLicense => "Microsoft Reciprocal License",
            SupportedLicense::GNUAfferoGeneralPublicLicenseV30OrLater => {
                "GNU Affero General Public License v3.0 or later"
            }
            SupportedLicense::EuropeanUnionPublicLicense11 => "European Union Public License 1.1",
            SupportedLicense::DoWhatTheFuckYouWantToPublicLicense => {
                "Do What The Fuck You Want To Public License"
            }
            SupportedLicense::ZlibLicense => "Zlib License",
            SupportedLicense::GNUAfferoGeneralPublicLicenseV20OrLaterWithAutoconfException => {
                "GNU Affero General Public License v2.0 or later with Autoconf exception"
            }
            SupportedLicense::TheUnlicense => "The Unlicense",
            SupportedLicense::GNULesserGeneralPublicLicenseV21OrLater => {
                "GNU Lesser General Public License v2.1 or later"
            }
            SupportedLicense::GNULesserGeneralPublicLicenseV20 => {
                "GNU Lesser General Public License v2.0"
            }
            SupportedLicense::BoostSoftwareLicense10 => "Boost Software License 1.0",
            SupportedLicense::GNUGeneralPublicLicenseV30Only => {
                "GNU General Public License v3.0 only"
            }
            SupportedLicense::GNULesserGeneralPublicLicenseV30Only => {
                "GNU Lesser General Public License v3.0 only"
            }
            SupportedLicense::BSD3ClauseClear => "BSD 3-Clause Clear",
            SupportedLicense::BSD4Clause => "BSD 4-Clause",
            SupportedLicense::GNUGeneralPublicLicenseV30OrLaterWithAutoconfException => {
                "GNU General Public License v3.0 or later with Autoconf exception"
            }
            SupportedLicense::EuropeanUnionPublicLicense12 => "European Union Public License 1.2",
            SupportedLicense::BSD3ClauseOriginal => "BSD 3-Clause Original",
            SupportedLicense::SILOpenFontLicense11 => "SIL Open Font License 1.1",
            SupportedLicense::GNUGeneralPublicLicenseV10 => "GNU General Public License v1.0",
            SupportedLicense::PostgreSQLLicense => "PostgreSQL License",
            SupportedLicense::ArtisticLicense20 => "Artistic License 2.0",
            SupportedLicense::ISCLicense => "ISC License",
            SupportedLicense::BSD2ClauseFreeBSD => "BSD 2-Clause FreeBSD",
            SupportedLicense::BSD3ClauseNew => "BSD 3-Clause New",
            SupportedLicense::BSD3ClauseMultiUse => "BSD 3-Clause Multi-Use",
            SupportedLicense::ApacheLicense20WithGCCException => {
                "Apache License 2.0 with GCC Exception"
            }
            SupportedLicense::GNUGeneralPublicLicenseV10OrLater => {
                "GNU General Public License v1.0 or later"
            }
            SupportedLicense::BSD3ClauseLBNL => "BSD 3-Clause LBNL",
            SupportedLicense::BSD3ClauseClearNew => "BSD 3-Clause Clear New",
            SupportedLicense::BSD3ClauseNoNuclearLicense => "BSD 3-Clause No Nuclear License",
            SupportedLicense::BSD3ClauseOpenSSL => "BSD 3-Clause OpenSSL",
            SupportedLicense::BSD3ClauseAttribution => "BSD 3-Clause Attribution",
            SupportedLicense::BSD4ClauseUC => "BSD 4-Clause UC",
            SupportedLicense::GNUGeneralPublicLicenseV20OrLaterWithBisonException => {
                "GNU General Public License v2.0 or later with Bison exception"
            }
            SupportedLicense::GNULesserGeneralPublicLicenseV21OrLaterWithGCCException => {
                "GNU Lesser General Public License v2.1 or later with GCC exception"
            }
            SupportedLicense::ZlibLicenseOnly => "Zlib License Only",
            SupportedLicense::BSD3ClauseLimited => "BSD 3-Clause Limited",
            SupportedLicense::BSD3ClauseRuby => "BSD 3-Clause Ruby",
            SupportedLicense::BSD3ClauseUniversityOfIllinois => {
                "BSD 3-Clause University of Illinois"
            }
            SupportedLicense::GNULesserGeneralPublicLicenseV21OrLaterWithClasspathException => {
                "GNU Lesser General Public License v2.1 or later with Classpath exception"
            }
            SupportedLicense::BSD3ClauseUCBerkeley => "BSD 3-Clause U.C. Berkeley",
            SupportedLicense::MicrosoftPublicLicense20 => "Microsoft Public License 2.0",
            SupportedLicense::ApacheLicense11 => "Apache License 1.1",
            SupportedLicense::BSD3ClauseRevised => "BSD 3-Clause Revised",
            SupportedLicense::GNUGeneralPublicLicenseV20WithAutoconfException => {
                "GNU General Public License v2.0 with Autoconf exception"
            }
            SupportedLicense::ApacheLicense20WithLLVMException => {
                "Apache License 2.0 with LLVM Exception"
            }
            SupportedLicense::BlueOakModelLicense100 => "BlueOak Model License 1.0.0",
            SupportedLicense::CC010Universal => "CC0 1.0 Universal",
            SupportedLicense::CreativeCommonsAttributionShareAlike40International => {
                "Creative Commons Attribution Share Alike 4.0 International"
            }
        }
        .to_string()
    }
}

impl FromStr for SupportedLicense {
    type Err = ();

    /// Tries to parse a string into a `SupportedLicense`.
    ///
    /// Uses a case-insensitive search for the string among some defined keywords for each license.
    ///
    /// ## First example
    /// The following:
    /// - "Apache-2.0"
    /// - "apache2"
    /// - "apache-2"
    ///
    /// will all be parsed into `SupportedLicense::Apache20`.
    ///
    /// ## Second example
    /// The following:
    /// - "cc-by-4.0",
    /// - "Attribution-ShareAlike 4.0",
    /// - "ccby4.0",
    /// - "ccby4",
    /// - "ccby",
    ///
    /// will all be parsed into `SupportedLicense::CreativeCommonsAttributionShareAlike40International`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        let mut keywords: HashMap<Vec<String>, SupportedLicense> = HashMap::new();

        keywords.insert(
            vec![
                SupportedLicense::Apache20.to_string(),
                "apache2".to_string(),
                "apache-2.0".to_string(),
                "apache-2".to_string(),
                "apache2.0".to_string(),
            ],
            SupportedLicense::Apache20,
        );
        keywords.insert(vec!["mit".to_string()], SupportedLicense::MIT);
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV30.to_string(),
                "gplv3".to_string(),
                "gpl-3.0".to_string(),
                "gpl-3".to_string(),
                "gplv3.0".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV30,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD2Clause.to_string(),
                "bsd2".to_string(),
                "bsd-2.0".to_string(),
                "bsd-2".to_string(),
                "bsd2.0".to_string(),
            ],
            SupportedLicense::BSD2Clause,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3Clause.to_string(),
                "bsd3".to_string(),
                "bsd-3.0".to_string(),
                "bsd-3".to_string(),
                "bsd3.0".to_string(),
            ],
            SupportedLicense::BSD3Clause,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNULesserGeneralPublicLicenseV30.to_string(),
                "lgplv3".to_string(),
                "lgpl-3.0".to_string(),
                "lgpl-3".to_string(),
                "lgplv3.0".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV30,
        );
        keywords.insert(
            vec![
                SupportedLicense::MozillaPublicLicense20.to_string(),
                "mplv2".to_string(),
                "mpl-2.0".to_string(),
                "mpl-2".to_string(),
                "mplv2.0".to_string(),
            ],
            SupportedLicense::MozillaPublicLicense20,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUAfferoGeneralPublicLicenseV30.to_string(),
                "agplv3".to_string(),
                "agpl-3.0".to_string(),
                "agpl-3".to_string(),
                "agplv3.0".to_string(),
            ],
            SupportedLicense::GNUAfferoGeneralPublicLicenseV30,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV20.to_string(),
                "gplv2".to_string(),
                "gpl-2.0".to_string(),
                "gpl-2".to_string(),
                "gplv2.0".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV20,
        );
        keywords.insert(
            vec![
                SupportedLicense::EclipsePublicLicense20.to_string(),
                "eplv2".to_string(),
                "epl-2.0".to_string(),
                "epl-2".to_string(),
                "eplv2.0".to_string(),
            ],
            SupportedLicense::EclipsePublicLicense20,
        );
        keywords.insert(
            vec![
                SupportedLicense::Unlicense.to_string(),
                "unlicense".to_string(),
            ],
            SupportedLicense::Unlicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::CreativeCommonsZeroV10Universal.to_string(),
                "cc0".to_string(),
            ],
            SupportedLicense::CreativeCommonsZeroV10Universal,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV20OrLater.to_string(),
                "gplv2+".to_string(),
                "gpl-2.0+".to_string(),
                "gpl-2+".to_string(),
                "gplv2.0+".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV20OrLater,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUAfferoGeneralPublicLicenseV10.to_string(),
                "agplv1".to_string(),
                "agpl-1.0".to_string(),
                "agplv1.0".to_string(),
            ],
            SupportedLicense::GNUAfferoGeneralPublicLicenseV10,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV10.to_string(),
                "lgplv2".to_string(),
                "lgpl-2.0".to_string(),
                "lgpl-2".to_string(),
                "lgplv2.0".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV21,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV20OrLater.to_string(),
                "lgplv2+".to_string(),
                "lgpl-2.0+".to_string(),
                "lgpl-2+".to_string(),
                "lgplv2.0+".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV20OrLater,
        );
        keywords.insert(
            vec![SupportedLicense::ISC.to_string(), "isc".to_string()],
            SupportedLicense::ISC,
        );
        keywords.insert(
            vec![
                SupportedLicense::MicrosoftPublicLicense.to_string(),
                "mspl".to_string(),
                "ms-pl".to_string(),
            ],
            SupportedLicense::MicrosoftPublicLicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::MicrosoftReciprocalLicense.to_string(),
                "msrl".to_string(),
                "ms-rl".to_string(),
            ],
            SupportedLicense::MicrosoftReciprocalLicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUAfferoGeneralPublicLicenseV30OrLater.to_string(),
                "agplv3+".to_string(),
                "agpl-3.0+".to_string(),
                "agpl-3+".to_string(),
                "agplv3.0+".to_string(),
            ],
            SupportedLicense::GNUAfferoGeneralPublicLicenseV30OrLater,
        );
        keywords.insert(
            vec![
                SupportedLicense::EuropeanUnionPublicLicense11.to_string(),
                "euplv1.1".to_string(),
                "eupl-1.1".to_string(),
                "euplv1.1".to_string(),
            ],
            SupportedLicense::EuropeanUnionPublicLicense11,
        );
        keywords.insert(
            vec![
                SupportedLicense::DoWhatTheFuckYouWantToPublicLicense.to_string(),
                "wtfpl".to_string(),
            ],
            SupportedLicense::DoWhatTheFuckYouWantToPublicLicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::ZlibLicense.to_string(),
                "zlib".to_string(),
            ],
            SupportedLicense::ZlibLicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::TheUnlicense.to_string(),
                "unlicense".to_string(),
            ],
            SupportedLicense::TheUnlicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNULesserGeneralPublicLicenseV21OrLater.to_string(),
                "lgplv2.1".to_string(),
                "lgpl-2.1".to_string(),
                "lgplv2.1".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV21OrLater,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNULesserGeneralPublicLicenseV20.to_string(),
                "lgplv2.0".to_string(),
                "lgpl-2.0".to_string(),
                "lgplv2.0".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV20,
        );
        keywords.insert(
            vec![
                SupportedLicense::BoostSoftwareLicense10.to_string(),
                "bsl".to_string(),
                "boost".to_string(),
            ],
            SupportedLicense::BoostSoftwareLicense10,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV30Only.to_string(),
                "gplv3only".to_string(),
                "gpl-3.0-only".to_string(),
                "gpl-3-only".to_string(),
                "gplv3.0-only".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV30Only,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNULesserGeneralPublicLicenseV30Only.to_string(),
                "lgplv3only".to_string(),
                "lgpl-3.0-only".to_string(),
                "lgpl-3-only".to_string(),
                "lgplv3.0-only".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV30Only,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseClear.to_string(),
                "bsd3clear".to_string(),
                "bsd-3-clear".to_string(),
                "bsd-3-clear-new".to_string(),
                "bsd3-clear".to_string(),
                "bsd3-clear-new".to_string(),
            ],
            SupportedLicense::BSD3ClauseClear,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD4Clause.to_string(),
                "bsd4".to_string(),
                "bsd-4.0".to_string(),
                "bsd-4".to_string(),
                "bsd4.0".to_string(),
            ],
            SupportedLicense::BSD4Clause,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV30OrLaterWithAutoconfException
                    .to_string(),
                "gplv3+autoconf".to_string(),
                "gpl-3.0+autoconf".to_string(),
                "gpl-3+autoconf".to_string(),
                "gplv3.0+autoconf".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV30OrLaterWithAutoconfException,
        );
        keywords.insert(
            vec![
                SupportedLicense::EuropeanUnionPublicLicense12.to_string(),
                "euplv1.2".to_string(),
                "eupl-1.2".to_string(),
                "euplv1.2".to_string(),
            ],
            SupportedLicense::EuropeanUnionPublicLicense12,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseOriginal.to_string(),
                "bsd3original".to_string(),
                "bsd-3-original".to_string(),
                "bsd3-original".to_string(),
            ],
            SupportedLicense::BSD3ClauseOriginal,
        );
        keywords.insert(
            vec![
                SupportedLicense::SILOpenFontLicense11.to_string(),
                "silofl".to_string(),
                "ofl".to_string(),
                "sil-open-font-license".to_string(),
            ],
            SupportedLicense::SILOpenFontLicense11,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV10.to_string(),
                "gplv1".to_string(),
                "gpl-1.0".to_string(),
                "gpl-1".to_string(),
                "gplv1.0".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV10,
        );
        keywords.insert(
            vec![
                SupportedLicense::PostgreSQLLicense.to_string(),
                "postgresql".to_string(),
                "postgresql-license".to_string(),
            ],
            SupportedLicense::PostgreSQLLicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::ArtisticLicense20.to_string(),
                "artisticv2".to_string(),
                "artistic-2.0".to_string(),
                "artistic-2".to_string(),
                "artisticv2.0".to_string(),
            ],
            SupportedLicense::ArtisticLicense20,
        );
        keywords.insert(
            vec![SupportedLicense::ISCLicense.to_string(), "isc".to_string()],
            SupportedLicense::ISCLicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD2ClauseFreeBSD.to_string(),
                "bsd2freebsd".to_string(),
                "bsd-2-freebsd".to_string(),
                "bsd2-freebsd".to_string(),
            ],
            SupportedLicense::BSD2ClauseFreeBSD,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseNew.to_string(),
                "bsd3new".to_string(),
                "bsd-3-new".to_string(),
                "bsd3-new".to_string(),
            ],
            SupportedLicense::BSD3ClauseNew,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseMultiUse.to_string(),
                "bsd3multiuse".to_string(),
                "bsd-3-multi-use".to_string(),
                "bsd3-multi-use".to_string(),
            ],
            SupportedLicense::BSD3ClauseMultiUse,
        );
        keywords.insert(
            vec![
                SupportedLicense::ApacheLicense20WithGCCException.to_string(),
                "apache2+gcc".to_string(),
                "apache-2.0+gcc".to_string(),
                "apache-2+gcc".to_string(),
                "apache2.0+gcc".to_string(),
            ],
            SupportedLicense::ApacheLicense20WithGCCException,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV10OrLater.to_string(),
                "gplv1+".to_string(),
                "gpl-1.0+".to_string(),
                "gpl-1+".to_string(),
                "gplv1.0+".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV10OrLater,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseLBNL.to_string(),
                "bsd3lbnl".to_string(),
                "bsd-3-lbnl".to_string(),
                "bsd3-lbnl".to_string(),
            ],
            SupportedLicense::BSD3ClauseLBNL,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseClearNew.to_string(),
                "bsd3clearnew".to_string(),
                "bsd-3-clear-new".to_string(),
                "bsd3-clear-new".to_string(),
            ],
            SupportedLicense::BSD3ClauseClearNew,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseNoNuclearLicense.to_string(),
                "bsd3nonuclear".to_string(),
                "bsd-3-no-nuclear".to_string(),
                "bsd3-no-nuclear".to_string(),
            ],
            SupportedLicense::BSD3ClauseNoNuclearLicense,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseOpenSSL.to_string(),
                "bsd3openssl".to_string(),
                "bsd-3-openssl".to_string(),
                "bsd3-openssl".to_string(),
            ],
            SupportedLicense::BSD3ClauseOpenSSL,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseAttribution.to_string(),
                "bsd3attribution".to_string(),
                "bsd-3-attribution".to_string(),
                "bsd3-attribution".to_string(),
            ],
            SupportedLicense::BSD3ClauseAttribution,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD4ClauseUC.to_string(),
                "bsd4uc".to_string(),
                "bsd-4-uc".to_string(),
                "bsd4-uc".to_string(),
            ],
            SupportedLicense::BSD4ClauseUC,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV20OrLaterWithBisonException.to_string(),
                "gplv2+bison".to_string(),
                "gpl-2.0+bison".to_string(),
                "gpl-2+bison".to_string(),
                "gplv2.0+bison".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV20OrLaterWithBisonException,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNULesserGeneralPublicLicenseV21OrLaterWithGCCException
                    .to_string(),
                "lgplv2.1+gcc".to_string(),
                "lgpl-2.1+gcc".to_string(),
                "lgplv2.1+gcc".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV21OrLaterWithGCCException,
        );
        keywords.insert(
            vec![
                SupportedLicense::ZlibLicenseOnly.to_string(),
                "zlibonly".to_string(),
                "zlib-only".to_string(),
            ],
            SupportedLicense::ZlibLicenseOnly,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseLimited.to_string(),
                "bsd3limited".to_string(),
                "bsd-3-limited".to_string(),
                "bsd3-limited".to_string(),
            ],
            SupportedLicense::BSD3ClauseLimited,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseRuby.to_string(),
                "bsd3ruby".to_string(),
                "bsd-3-ruby".to_string(),
                "bsd3-ruby".to_string(),
            ],
            SupportedLicense::BSD3ClauseRuby,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseUniversityOfIllinois.to_string(),
                "bsd3uiuc".to_string(),
                "bsd-3-uiuc".to_string(),
                "bsd3-uiuc".to_string(),
            ],
            SupportedLicense::BSD3ClauseUniversityOfIllinois,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNULesserGeneralPublicLicenseV21OrLaterWithClasspathException
                    .to_string(),
                "lgplv2.1+classpath".to_string(),
                "lgpl-2.1+classpath".to_string(),
                "lgplv2.1+classpath".to_string(),
            ],
            SupportedLicense::GNULesserGeneralPublicLicenseV21OrLaterWithClasspathException,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseUCBerkeley.to_string(),
                "bsd3ucb".to_string(),
                "bsd-3-ucb".to_string(),
                "bsd3-ucb".to_string(),
            ],
            SupportedLicense::BSD3ClauseUCBerkeley,
        );
        keywords.insert(
            vec![
                SupportedLicense::MicrosoftPublicLicense20.to_string(),
                "msplv2".to_string(),
                "ms-pl-2.0".to_string(),
                "ms-pl-2".to_string(),
                "msplv2.0".to_string(),
            ],
            SupportedLicense::MicrosoftPublicLicense20,
        );
        keywords.insert(
            vec![
                SupportedLicense::ApacheLicense11.to_string(),
                "apache1".to_string(),
                "apache-1.0".to_string(),
                "apache1.0".to_string(),
            ],
            SupportedLicense::ApacheLicense11,
        );
        keywords.insert(
            vec![
                SupportedLicense::BSD3ClauseRevised.to_string(),
                "bsd3revised".to_string(),
                "bsd-3-revised".to_string(),
                "bsd3-revised".to_string(),
            ],
            SupportedLicense::BSD3ClauseRevised,
        );
        keywords.insert(
            vec![
                SupportedLicense::GNUGeneralPublicLicenseV20WithAutoconfException.to_string(),
                "gplv2+autoconf".to_string(),
                "gpl-2.0+autoconf".to_string(),
                "gpl-2+autoconf".to_string(),
                "gplv2.0+autoconf".to_string(),
            ],
            SupportedLicense::GNUGeneralPublicLicenseV20WithAutoconfException,
        );
        keywords.insert(
            vec![
                SupportedLicense::ApacheLicense20WithLLVMException.to_string(),
                "apache2+llvm".to_string(),
                "apache-2.0+llvm".to_string(),
                "apache-2+llvm".to_string(),
                "apache2.0+llvm".to_string(),
            ],
            SupportedLicense::ApacheLicense20WithLLVMException,
        );
        keywords.insert(
            vec![
                SupportedLicense::BlueOakModelLicense100.to_string(),
                "blueoak".to_string(),
            ],
            SupportedLicense::BlueOakModelLicense100,
        );
        keywords.insert(
            vec![
                SupportedLicense::CC010Universal.to_string(),
                "cc0-1.0".to_string(),
                "cc0v1.0".to_string(),
                "cc0v1".to_string(),
            ],
            SupportedLicense::CC010Universal,
        );
        keywords.insert(
            vec![
                SupportedLicense::CreativeCommonsAttributionShareAlike40International.to_string(),
                "cc-by-4.0".to_string(),
                "Attribution ShareAlike 4.0".to_string(),
                "ShareAlike 4.0".to_string(),
                "ccby4.0".to_string(),
                "ccby4".to_string(),
                "ccby".to_string(),
            ],
            SupportedLicense::CreativeCommonsAttributionShareAlike40International,
        );

        // check if contains the license
        let found = keywords.iter().find(|option| {
            option
                .0
                .iter()
                .any(|keyword| s.to_lowercase().contains(&keyword.to_lowercase()))
        });

        if let Some((_, license)) = found {
            Ok(*license)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// The license object and related information
pub struct License {
    /// The license name
    pub name: SupportedLicense,
    /// The location in the project structure
    pub path: Option<String>,
    /// The location on the web (github repository for example)
    pub url: Option<String>,
}

impl License {
    /// Create a new license object from a name (string)
    ///
    /// If the name of the license is not recognized, the license will be set to `SupportedLicense::Unknown`
    pub fn from_name(name: String) -> Self {
        let license = match SupportedLicense::from_str(&name) {
            Ok(license) => license,
            Err(_) => {
                return Self {
                    name: SupportedLicense::Unknown,
                    path: None,
                    url: None,
                }
            }
        };

        Self {
            name: license,
            path: None,
            url: None,
        }
    }

    /// Create a new license object from a file
    ///
    /// If the license file is from a known platform (eg: github), the url will be set to the file location on the platform
    pub fn from_file(path: String, repository: Option<Repository>) -> Self {
        let content = match fs::read_to_string(&path) {
            Ok(license) => license,
            Err(_) => {
                return Self {
                    name: SupportedLicense::Unknown,
                    path: None,
                    url: None,
                }
            }
        };

        let is_remote = repository.is_some()
            && repository.as_ref().unwrap().platform == RepositoryPlatform::Github;

        match SupportedLicense::from_str(&content) {
            Ok(license) => {
                if is_remote {
                    let filename = path.split('/').last().unwrap();

                    // repository.clone().unwrap().url.clone() + "/blob/master/" + filename,
                    let url = format!(
                        "{}/blob/master/{}",
                        repository.as_ref().unwrap().url,
                        filename
                    );

                    return Self {
                        name: license,
                        path: Some(path.clone()),
                        url: Some(url),
                    };
                }

                Self {
                    name: license,
                    path: Some(path),
                    url: None,
                }
            }
            Err(_) => Self {
                name: SupportedLicense::Unknown,
                path: Some(path),
                url: None,
            },
        }
    }
}

impl GenMarkdown for License {
    fn gen_md(&self) -> Result<String, anyhow::Error> {
        let license_tpl = paths::read_util_file_contents(paths::UtilityPath::License);
        let mut handlebars = handlebars::Handlebars::new();
        handlebars.register_template_string("license_tpl", license_tpl)?;

        let data = if let Some(url) = &self.url {
            json!({
                "name": self.name.to_string(),
                "target": url.clone()
            })
        } else {
            json!({
                "name": self.name.to_string()
            })
        };

        Ok(handlebars.render("license_tpl", &data).unwrap())
    }
}

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _license = String::new();

        if self.path.is_some() {
            return write!(
                f,
                "{} ({})",
                self.name.to_string(),
                self.path.clone().unwrap()
            );
        };

        write!(f, "{}", self.name.to_string())
    }
}
