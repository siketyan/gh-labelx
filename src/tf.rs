use anyhow::Result;
use hcl::{Block, BlockBuilder, BodyBuilder, Expression, Object, RawExpression};

use crate::manifest::Label;
use crate::Manifest;

impl From<&Label> for Block {
    fn from(label: &Label) -> Self {
        let mut builder = BlockBuilder::new("resource")
            .add_labels(vec!["github_issue_label", &label.name])
            .add_attribute(("repository", RawExpression::new("local.repo")))
            .add_attribute(("name", Expression::from(label.name.as_str())));

        if let Some(description) = label.description.as_deref() {
            builder = builder.add_attribute(("description", Expression::from(description)));
        }

        if let Some(color) = label.color.as_deref() {
            builder = builder.add_attribute(("color", Expression::from(color)));
        }

        builder.build()
    }
}

impl Manifest {
    pub fn generate_tf(&self, owner: &str, repo: &str) -> Result<String> {
        let blocks = self.labels.iter().map(|l| l.into()).collect::<Vec<Block>>();

        let body = BodyBuilder::default()
            .add_block(
                BlockBuilder::new("terraform")
                    .add_block(
                        BlockBuilder::new("required_providers")
                            .add_attribute((
                                "github",
                                Expression::Object({
                                    let mut obj = Object::new();
                                    obj.insert("source".into(), "integrations/github".into());
                                    obj.insert("version".into(), "~> 4.0".into());
                                    obj
                                }),
                            ))
                            .build(),
                    )
                    .build(),
            )
            .add_block(
                BlockBuilder::new("provider")
                    .add_label("github")
                    .add_attribute(("owner", Expression::from(owner)))
                    .build(),
            )
            .add_block(
                BlockBuilder::new("locals")
                    .add_attribute(("repo", Expression::from(repo)))
                    .build(),
            )
            .add_blocks(blocks)
            .build();

        Ok(hcl::to_string(&body)?)
    }
}
